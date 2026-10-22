# apps/

Drop each finished program here, one folder per product, then update
`releases.json`. The Schedulo dashboard downloads it from here on its next
start. Only compiled programs go here, never source code, and never the
Licence Manager.

| Folder | Product |
| --- | --- |
| `TXC/` | Schedulo TXC |
| `BUS/` | Schedulo Bus |
| `COACH/` | Schedulo Coach |
| `COSTING/` | Schedulo Costing |
| `NSA/` | Schedulo NSA |
| `FLEET/` | Schedulo Fleet |
| `WORKSHOP/` | Schedulo Workshop |
| `ROSTER/` | Schedulo Roster |
| `DRIVER/` | Schedulo Driver |
| `TM/` | Schedulo TM |

To publish a new version of, say, TXC:

1. Copy the new `.exe` to `apps/TXC/` (e.g. `apps/TXC/Schedulo-TXC.exe`).
2. In `releases.json`, set the TXC entry's `"version"` (e.g. `"0.5.1"`),
   `"released"` (`"2026-11-01"`) and `"file"` (`"apps/TXC/Schedulo-TXC.exe"`).
3. Commit and push. Everyone's Schedulo installs it the next time it opens.

GitHub refuses single files over 100 MB; Schedulo programs are much smaller.
