import { beforeEach, describe, expect, it, vi } from "vitest";
import { commands, type ProjectOffer } from "@/bindings";
import { routesFor, useCommitOfferStore } from "./commit-offer";

vi.mock("@/bindings", () => ({ commands: { commitOfferScan: vi.fn() } }));
vi.mock("sonner", () => ({ toast: { info: vi.fn(), success: vi.fn() } }));

/** A project with every choice standing: a remote chosen, `gh` answering,
 *  and no pull request open for the branch. */
const offer = (over: Partial<ProjectOffer> = {}): ProjectOffer => ({
  root: "/home/method/dev/site",
  name: "site",
  files: [".claude/CLAUDE.md", ".kendex-generated.json"],
  shared: [],
  others: 0,
  branch: "main",
  remote: "origin",
  push: null,
  pullRequest: null,
  openNumber: null,
  message: "chore: kendex refresh",
  newBranch: "kendex/renders",
  repo: "acme/site",
  tracked: true,
  ...over,
});

// The rows of the design's state table that decide which segments the
// offer draws, `docs/design/post-refresh-commit-flow.md` § State table.
describe("the choices an offer carries", () => {
  it("offers all three where nothing removed one", () => {
    expect(routesFor(offer())).toEqual(["commit", "push", "pr"]);
  });

  it("offers commit only with no remote", () => {
    expect(
      routesFor(
        offer({
          remote: null,
          repo: null,
          push: { kind: "noRemote" },
          pullRequest: { kind: "noRemote" },
        }),
      ),
    ).toEqual(["commit"]);
  });

  it("keeps the push where only gh is missing", () => {
    expect(
      routesFor(offer({ repo: null, pullRequest: { kind: "ghMissing" } })),
    ).toEqual(["commit", "push"]);
  });

  it("drops the pull request where one is already open", () => {
    expect(routesFor(offer({ openNumber: 41 }))).toEqual(["commit", "push"]);
  });
});

// kendex asks at most once per run per project: a project already in the
// line keeps its place, and leaving takes it off the line.
describe("the line of projects to ask", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useCommitOfferStore.setState({
      queue: [],
      flagged: [],
      stage: { at: "offer" },
      route: "commit",
      message: "",
    });
  });

  it("asks each project once, whatever the write reached it twice", async () => {
    vi.mocked(commands.commitOfferScan).mockResolvedValue({
      status: "ok",
      data: { offers: [offer()], flagged: [] },
    });
    const { enqueue } = useCommitOfferStore.getState();
    await enqueue(["/home/method/dev/site"]);
    await enqueue(["/home/method/dev/site"]);
    const state = useCommitOfferStore.getState();
    expect(state.queue.map((each) => each.root)).toEqual([
      "/home/method/dev/site",
    ]);
    expect(state.message).toBe("chore: kendex refresh");
    expect(commands.commitOfferScan).toHaveBeenCalledTimes(2);
  });

  it("asks nothing when the write could reach no project", async () => {
    await useCommitOfferStore.getState().enqueue([]);
    expect(commands.commitOfferScan).not.toHaveBeenCalled();
  });

  it("leaves the files as diffs and moves to the next project", async () => {
    const second = offer({ root: "/home/method/dev/other", name: "other" });
    vi.mocked(commands.commitOfferScan).mockResolvedValue({
      status: "ok",
      data: { offers: [offer(), second], flagged: [] },
    });
    await useCommitOfferStore.getState().enqueue(["/a", "/b"]);
    useCommitOfferStore.getState().pick("pr");
    useCommitOfferStore.getState().leave();
    const state = useCommitOfferStore.getState();
    expect(state.queue.map((each) => each.root)).toEqual([
      "/home/method/dev/other",
    ]);
    expect(state.stage).toEqual({ at: "offer" });
    expect(state.route).toBe("commit");
    useCommitOfferStore.getState().leave();
    expect(useCommitOfferStore.getState().queue).toEqual([]);
  });
});
