# Audit Project Order Output Schema

Output path: `tmp/audit-project-order-YYYYMMDD-HHMMSS.json`

## Schema

```json
{
  "generated": "ISO timestamp",
  "initiatives": [
    {"id": "uuid", "name": "Platform MVP"}
  ],
  "projects_analyzed": [
    {
      "id": "uuid",
      "name": "string",
      "state": "backlog|planned|started|completed|paused",
      "initiative": "Platform MVP",
      "current_sort_order": -5000,
      "current_position_in_state": 3,
      "layer": 1,
      "domain": "data|infra|ui|testing",
      "deliverables": ["Message queue", "API gateway"],
      "consumes": ["Foundation types"],
      "analysis": "2-3 sentence architectural rationale"
    }
  ],
  "recommended_order": [
    {
      "position": 1,
      "project_id": "uuid",
      "name": "string",
      "initiative": "Platform MVP",
      "layer": 0,
      "target_state": "planned",
      "new_sort_order": -10000
    }
  ],
  "reorder": [
    {
      "project_id": "uuid",
      "name": "string",
      "initiative": "Platform MVP",
      "current_state": "backlog",
      "target_state": "backlog",
      "current_position_in_state": 3,
      "recommended_position_in_state": 1,
      "current_sort_order": -5000,
      "new_sort_order": -10000,
      "rationale": "L1 layer should be first in backlog, before L2+ projects"
    }
  ],
  "complete_candidates": [
    {"id": "uuid", "name": "string", "progress": 1.0, "unblocks": ["Project X"]}
  ],
  "recommended_next": {
    "id": "uuid or null",
    "name": "string or null",
    "rationale": "Position 1 after reorder, no incomplete blockers, L1 builds on completed L0"
  },
  "summary": {
    "projects_analyzed": 0,
    "reorder_needed": 0,
    "projects_to_complete": 0,
    "projects_ready": 0
  }
}
```

## Field Definitions

### Layer

Architectural layer for dependency ordering:
- `0`: Foundation (no dependencies)
- `1`: Core infrastructure (builds on L0)
- `2`: Features (builds on L1)
- `3+`: Higher-level features

### Domain

Functional area:
- `infra`: Build, tooling, CI/CD
- `data`: Data layer, storage, IPC
- `ui`: User interface, rendering
- `testing`: Test infrastructure

### State

Issue tracker project state:
- `backlog`: Not yet scheduled
- `planned`: Scheduled for future
- `started`: Currently active
- `completed`: Finished
- `paused`: On hold
