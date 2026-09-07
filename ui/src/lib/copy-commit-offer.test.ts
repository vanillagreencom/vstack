import { describe, expect, it } from "vitest";
import type { Refused } from "@/bindings";
import {
  addsToPullRequest,
  BRANCH_REFUSED_LINE,
  BRANCH_REFUSED_TITLE,
  BRANCH_ROW_LABEL,
  backOn,
  branchIsOn,
  COMMIT_AGAIN_LABEL,
  COMMIT_LABEL,
  COMMIT_OFFER_SETTING_DESCRIPTION,
  COMMIT_OFFER_SETTING_LABEL,
  COMMIT_OFFER_STANDING,
  COMMIT_ROW_LABEL,
  COMMIT_SEGMENT,
  COMMIT_STAYS,
  COMMITTING_LABEL,
  commitIsOn,
  commitOfferTitle,
  commitOn,
  commitRefusedTitle,
  committedToast,
  DONE_LABEL,
  didNotFinish,
  FILES_LABEL,
  GH_SAID_LABEL,
  GIT_PROJECTS_SECTION,
  GIT_SAID_LABEL,
  LEAVE_IT_HERE_LABEL,
  LEAVE_LABEL,
  MESSAGE_LABEL,
  NOT_CHECKED_BADGE,
  NOT_PUT_BACK_LINE,
  NOT_PUT_BACK_TITLE,
  NOTHING_TO_COMMIT_TOAST,
  notChecked,
  nowOn,
  OPEN_PR_LABEL,
  OPENING_LABEL,
  OTHER_LABEL,
  otherNote,
  PR_LABEL,
  PR_OPEN_TITLE,
  PR_ROW_LABEL,
  PR_SEGMENT,
  PUSH_LABEL,
  PUSH_ROW_LABEL,
  PUSH_SEGMENT,
  PUSHING_LABEL,
  prMoves,
  pullRequestRefusedTitle,
  pushedToast,
  pushesTo,
  pushRefusedTitle,
  putBackRowLabel,
  remoteBranch,
  resetCommand,
  SHARED_LABEL,
  SHARED_NOTE,
  saidLabel,
  stillCarries,
  stillStaged,
  unavailableReason,
  uncommittedBadge,
  uncommittedInProgress,
  uncommittedNoBranch,
  WHAT_TO_DO_LABEL,
} from "./copy-commit-offer";

// The design's example values, `docs/design/post-refresh-commit-flow.md`
// § App: every string below is the one its table prints for them.
const FILES = 12;
const OTHERS = 4;
const PROJECT = "site";
const REMOTE = "origin";
const BRANCH = "main";
const NEW_BRANCH = "kendex/renders";
const SHA = "9fbb1a2";
const NUMBER = 41;
const BEFORE = "4c1d90e";
const SECONDS = 300;

const refused = (over: Partial<Refused> = {}): Refused => ({
  step: "the commit",
  said: ["commit-msg: crates/ changed without a changelog entry"],
  timedOut: false,
  seconds: SECONDS,
  gh: false,
  ...over,
});

describe("the offer state", () => {
  it("prints the design's words for its example values", () => {
    expect(commitOfferTitle(FILES, PROJECT)).toBe(
      "kendex changed 12 files in site",
    );
    expect(commitOfferTitle(1, PROJECT)).toBe("kendex changed 1 file in site");
    expect(COMMIT_OFFER_STANDING).toBe(
      "These are the files kendex writes in this repository. Nothing is committed yet.",
    );
    expect(FILES_LABEL).toBe("Files");
    expect(SHARED_LABEL).toBe("Shared files");
    expect(SHARED_NOTE).toBe(
      "kendex writes one key in each of these. Committing them would commit your own changes to them too, so kendex leaves them to you.",
    );
    expect(OTHER_LABEL).toBe("Other changes");
    expect(otherNote(OTHERS)).toBe(
      "4 other files in this repository changed. kendex does not commit these.",
    );
    expect(otherNote(1)).toBe(
      "1 other file in this repository changed. kendex does not commit these.",
    );
    expect(WHAT_TO_DO_LABEL).toBe("What to do");
    expect(COMMIT_SEGMENT).toBe("Commit");
    expect(PUSH_SEGMENT).toBe("Commit and push");
    expect(PR_SEGMENT).toBe("Pull request");
    expect(COMMIT_STAYS).toBe("The commit stays in this checkout.");
    expect(pushesTo(REMOTE, BRANCH)).toBe("Pushes to origin/main.");
    expect(addsToPullRequest(NUMBER)).toBe(
      "Adds a commit to pull request #41.",
    );
    expect(prMoves(NEW_BRANCH, BRANCH)).toBe(
      "Commits on kendex/renders and opens a pull request. This checkout moves to that branch. main stays where it is.",
    );
    expect(MESSAGE_LABEL).toBe("Message");
    expect(LEAVE_LABEL).toBe("Leave as diffs");
    expect(COMMIT_LABEL).toBe("Commit");
    expect(PUSH_LABEL).toBe("Commit and push");
    expect(PR_LABEL).toBe("Open pull request");
  });
});

describe("the rows for a segment a precondition removed", () => {
  // The `ghSaid` row is gh's own first line, not the design's fixed
  // `gh is not signed in. Run gh auth login.`: kendex does not decide what
  // gh meant.
  it("labels the row and names the reason", () => {
    expect(PUSH_ROW_LABEL).toBe("Push");
    expect(PR_ROW_LABEL).toBe("Pull request");
    expect(unavailableReason({ kind: "noRemote" })).toBe(
      "This repository has no remote.",
    );
    expect(unavailableReason({ kind: "remoteNotDecidable" })).toBe(
      "This branch tracks no remote and the repository has more than one.",
    );
    expect(unavailableReason({ kind: "ghMissing" })).toBe(
      "gh is not installed.",
    );
    expect(
      unavailableReason({
        kind: "ghSaid",
        line: "To get started with GitHub CLI, please run:  gh auth login",
      }),
    ).toBe("To get started with GitHub CLI, please run:  gh auth login");
  });
});

describe("the busy state", () => {
  it("names the step running, with an ellipsis", () => {
    expect(COMMITTING_LABEL).toBe("Committing…");
    expect(PUSHING_LABEL).toBe("Pushing…");
    expect(OPENING_LABEL).toBe("Opening the pull request…");
  });
});

describe("the result states", () => {
  it("toasts the two that close and draws the pull request", () => {
    expect(committedToast(FILES)).toBe("Committed 12 files");
    expect(committedToast(1)).toBe("Committed 1 file");
    expect(pushedToast(FILES)).toBe("Committed and pushed 12 files");
    expect(NOTHING_TO_COMMIT_TOAST).toBe("Nothing to commit");
    expect(PR_OPEN_TITLE).toBe("Pull request open");
    expect(COMMIT_ROW_LABEL).toBe("Commit");
    expect(commitOn(SHA, NEW_BRANCH)).toBe("9fbb1a2 on kendex/renders");
    expect(nowOn(NEW_BRANCH)).toBe("This checkout is now on kendex/renders.");
    expect(DONE_LABEL).toBe("Done");
    // The refused-push recovery's two added rows.
    expect(stillCarries(BRANCH)).toBe(
      "main in this checkout still carries the commit.",
    );
    expect(putBackRowLabel(BRANCH)).toBe("To put main back");
    expect(resetCommand(BEFORE)).toBe("git reset --mixed 4c1d90e");
  });
});

describe("the refusal states", () => {
  it("prints each title, section heading, line and footer", () => {
    expect(commitRefusedTitle(refused())).toBe("The commit was refused");
    expect(commitRefusedTitle(refused({ step: "the check" }))).toBe(
      "The files could not be checked",
    );
    expect(NOT_PUT_BACK_TITLE).toBe("The checkout could not be put back");
    expect(commitRefusedTitle(refused({ step: "the staging" }))).toBe(
      "The files could not be staged",
    );
    expect(GIT_SAID_LABEL).toBe("What git said");
    expect(GH_SAID_LABEL).toBe("What gh said");
    expect(saidLabel(refused())).toBe("What git said");
    expect(saidLabel(refused({ gh: true }))).toBe("What gh said");
    expect(COMMIT_AGAIN_LABEL).toBe("Commit again");
    expect(stillStaged(OTHERS)).toBe(
      "kendex staged 4 files it could not unstage. They are still staged.",
    );
    expect(BRANCH_REFUSED_TITLE).toBe("The branch could not be made");
    expect(BRANCH_REFUSED_LINE).toBe(
      "Nothing was committed and this checkout has not moved.",
    );
    expect(backOn(BRANCH, NEW_BRANCH)).toBe(
      "This checkout is back on main and kendex/renders is gone.",
    );
    expect(NOT_PUT_BACK_LINE).toBe("The checkout could not be put back.");
    expect(pushRefusedTitle(refused({ step: "the push" }))).toBe(
      "Committed, not pushed",
    );
    expect(commitOn(SHA, BRANCH)).toBe("9fbb1a2 on main");
    expect(commitIsOn(BRANCH)).toBe(
      "The commit is on main in this checkout. kendex did not undo it.",
    );
    expect(LEAVE_IT_HERE_LABEL).toBe("Leave it here");
    expect(OPEN_PR_LABEL).toBe("Open a pull request");
    expect(pullRequestRefusedTitle(refused({ step: "the pull request" }))).toBe(
      "Committed and pushed, no pull request",
    );
    expect(BRANCH_ROW_LABEL).toBe("Branch");
    expect(remoteBranch(REMOTE, NEW_BRANCH)).toBe("origin/kendex/renders");
    expect(branchIsOn(REMOTE)).toBe(
      "The branch is on origin. Open the pull request yourself.",
    );
  });
});

describe("a step that timed out", () => {
  it("turns the title's verb and replaces the section with one line", () => {
    const out = { timedOut: true, said: [] };
    expect(commitRefusedTitle(refused({ ...out, step: "the commit" }))).toBe(
      "The commit did not finish",
    );
    expect(pushRefusedTitle(refused({ ...out, step: "the push" }))).toBe(
      "The push did not finish",
    );
    expect(
      pullRequestRefusedTitle(refused({ ...out, step: "the pull request" })),
    ).toBe("The pull request did not finish");
    expect(didNotFinish(SECONDS)).toBe(
      "kendex stopped waiting after 300 seconds. Whether it finished is not known here.",
    );
    expect(didNotFinish(1)).toBe(
      "kendex stopped waiting after 1 second. Whether it finished is not known here.",
    );
  });
});

describe("the project card where no offer can be made", () => {
  it("draws the badge and puts the reason on hover", () => {
    expect(uncommittedBadge(FILES)).toBe("12 uncommitted");
    expect(uncommittedNoBranch(FILES)).toBe(
      "12 files kendex wrote are not committed. This checkout is on no branch.",
    );
    expect(uncommittedNoBranch(1)).toBe(
      "1 file kendex wrote is not committed. This checkout is on no branch.",
    );
    expect(uncommittedInProgress(FILES, "a rebase")).toBe(
      "12 files kendex wrote are not committed. A rebase is in progress.",
    );
    expect(NOT_CHECKED_BADGE).toBe("Not checked");
    expect(notChecked(["fatal: not a git repository"])).toBe(
      "kendex could not check the files it wrote here. git said: fatal: not a git repository",
    );
    expect(notChecked([])).toBe(
      "kendex could not check the files it wrote here.",
    );
  });
});

describe("the setting", () => {
  it("names the section, the row and what the switch does", () => {
    expect(GIT_PROJECTS_SECTION).toBe("Git projects");
    expect(COMMIT_OFFER_SETTING_LABEL).toBe("Offer to commit kendex's changes");
    expect(COMMIT_OFFER_SETTING_DESCRIPTION).toBe(
      "In a git project, ask what to do with the files kendex wrote.",
    );
  });
});
