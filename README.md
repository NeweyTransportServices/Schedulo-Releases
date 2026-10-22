# Schedulo releases

**Transport planning software by Newey Transport Services**
PLAN · OPERATE · MANAGE · COMPLY · GROW

This is the public home for Schedulo: the **Schedulo dashboard app** (source
in `src/`) and the finished Schedulo programs it installs (in `apps/`). It holds
compiled programs only, never the apps' source code. The Schedulo apps and the
dashboard read this repository to find new versions and revoked licence keys.

## The Schedulo dashboard

One program, `Schedulo.exe`. On every start it:

1. creates its folders (`%LOCALAPPDATA%\Schedulo\apps\<product>\` for the
   programs, and `%APPDATA%\Schedulo\` for the shared revoked-keys list);
2. reads `releases.json` here and downloads, from `apps/`, any product that is
   newer than the one installed (or not installed yet);
3. refreshes the owner-signed revoked-keys list for every Schedulo app;
4. shows every product as its brand tile with an **Open** button.

It needs no account or access token: everything it reads is public. Offline,
it opens the installed versions as normal.

Build it with `cargo build --release` (output `target/release/Schedulo.exe`).
To try a release before pushing it, set `SCHEDULO_RELEASES` to a local copy of
this repository and run Schedulo: it reads from that folder instead.

## Products

| | Product | | Latest version | Download |
| :-: | --- | --- | :-: | --- |
| <img src="assets/icons/txc.svg" width="40"> | **Schedulo TXC** | TransXChange | **0.5.0** | Coming soon |
| <img src="assets/icons/bus.svg" width="40"> | **Schedulo Bus** | Bus scheduling | — | Coming soon |
| <img src="assets/icons/coach.svg" width="40"> | **Schedulo Coach** | Coach scheduling | — | Coming soon |
| <img src="assets/icons/costing.svg" width="40"> | **Schedulo Costing** | Operational costing | — | Coming soon |
| <img src="assets/icons/nsa.svg" width="40"> | **Schedulo NSA** | Next Stop Announcements | — | Coming soon |
| <img src="assets/icons/fleet.svg" width="40"> | **Schedulo Fleet** | Vehicle & asset management | — | Coming soon |
| <img src="assets/icons/workshop.svg" width="40"> | **Schedulo Workshop** | Maintenance & workshop | — | Coming soon |
| <img src="assets/icons/roster.svg" width="40"> | **Schedulo Roster** | Crew rostering | — | Coming soon |
| <img src="assets/icons/driver.svg" width="40"> | **Schedulo Driver** | Driver management | — | Coming soon |
| <img src="assets/icons/tm.svg" width="40"> | **Schedulo TM** | Transport management | — | Coming soon |

Downloads are on the [Releases page](../../releases) once published. Each
release is a zip: unzip it anywhere and run the `.exe`. You need a licence key
for each product (or a bundle key that covers several).

## Licence keys

| | |
| --- | --- |
| **Revoked keys** | [`licence/revoked.txt`](licence/revoked.txt), signed by the owner (published from the Licence Manager) |
| **What it contains** | Key ids only: no names, companies or keys |
| **Who reads it** | Every Schedulo app and the Schedulo dashboard, which check the signature before using it |

## For developers

The two files apps read, and how they must use them, are described in
[docs/FORMAT.md](docs/FORMAT.md). All Schedulo apps must follow it.

To publish a new version by hand:

1. Copy the program into its folder in [`apps/`](apps/README.md), e.g.
   `apps/TXC/Schedulo-TXC.exe`.
2. Update that product's entry in [`releases.json`](releases.json) (`version`,
   `released`, `file`) and the table above, then commit and push.
