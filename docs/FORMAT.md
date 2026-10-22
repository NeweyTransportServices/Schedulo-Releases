# Schedulo-Releases: file formats

Every Schedulo app (Schedulo TXC 0.5.0 onwards, the Schedulo dashboard and the
rest of the family) reads two files from this repository. This page is the
single definition; apps must not invent their own variants.

Raw base URL: `https://raw.githubusercontent.com/NeweyTransportServices/Schedulo-Releases/main/`

## 1. `releases.json`: latest versions

```json
{
  "schema": 1,
  "updated": "2026-10-22",
  "products": {
    "TXC": {
      "name": "Schedulo TXC",
      "descriptor": "TransXChange",
      "version": "0.5.0",
      "released": "2026-10-22",
      "file": "apps/TXC/Schedulo-TXC.exe",
      "download": null,
      "notes": "https://github.com/NeweyTransportServices/Schedulo-Releases/releases/tag/txc-v0.5.0"
    }
  }
}
```

- Keys under `products` are the short product labels from
  `nts_licence::Product::short()`: `TXC`, `BUS`, `COACH`, `COSTING`, `NSA`,
  `FLEET`, `WORKSHOP`, `ROSTER`, `DRIVER`, `TM`.
- `version` is `MAJOR.MINOR.PATCH` (no leading `v`); `null` means not
  released yet. `released` is `YYYY-MM-DD` or `null`. `file` is the program's
  path in this repository (under `apps/`), which the Schedulo dashboard
  downloads and installs; `null` means not available yet. `download` is an
  optional zip for manual download
  (or `null`); `notes` is a release-notes page (or `null`).
- An app compares its own `CARGO_PKG_VERSION` with its product's `version`
  and offers the update only when the published one is newer. Unknown fields
  are ignored; if `schema` is greater than 1, the app ignores the file.
- Any failure (offline, missing file, bad JSON) is silent: carry on as normal.
- The Licence Manager is never listed or published here.

## 2. `licence/revoked.txt`: revoked licence keys

Published from the Licence Manager (**Publish revocation list**, owner only).
Signed with the owner's root key, so it can be trusted however it arrives.

```text
NTS-REVOCATIONS-V1
issued: 1792380000
0011223344556677
8899aabbccddeeff
signature: <base64 Ed25519 signature>
```

- Ids are the 16-hex-character key ids (user keys and manager keys; revoking
  a manager key revokes every key that manager made).
- The root signs `"NTS-LICENCE-V2|REVOCATIONS|"`, then `issued` (Unix seconds,
  i64 big-endian), then each id as 8 bytes in the order listed.
- Code: use `nts_licence::revocations` (identical in every repository):
  `revocations::fetch(&config.root_public_key)` downloads and verifies;
  `Revocations::verify(text, &root_public_key)` checks text you already have.

How apps must use it:

1. **Verify** the signature against the app's built-in root public key. Ignore
   the file completely if it fails.
2. **Only ever add.** Keep the union of every verified list seen, stored
   locally (shared by all apps at `%APPDATA%\Schedulo\revoked.txt`, written in
   this same signed format by whichever app or the dashboard fetched the
   newest one, so each app can re-verify it). Never remove an id because a
   newer download lacks it, and ignore a list whose `issued` is older than the
   stored one.
3. **Keep the built-in list** (`licence/revoked.txt` compiled into the app) as
   well, so offline PCs still refuse keys revoked at build time.
4. Treat a key as revoked if its id, or its issuing manager's id, is in any of
   these lists.
5. Check at most once per start-up, in the background; never block or delay
   opening the app.
