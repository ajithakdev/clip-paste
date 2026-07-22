# Translation guide

clip-paste uses Fluent translation files stored in `i18n/<locale>/clip_paste.ftl`.

Fallback language configuration lives in `i18n.toml` and currently uses `en`.

## Current translations and contributors

- English — [@k33wee](https://github.com/k33wee)
- Italian — [@k33wee](https://github.com/k33wee)
- Portuguese — [@GuilhermeTerriaga](https://github.com/GuilhermeTerriaga)
- Czech — [@lorduskordus](https://github.com/lorduskordus)
- Ukrainian — [@Dymkom](https://github.com/Dymkom)
- Swedish — [@bittin](https://github.com/bittin)
- French — [@Thovi98](https://github.com/Thovi98)
- Polish — [@VandaLHJ](https://github.com/VandaLHJ)

## Adding a new translation

1. Create a new directory under `i18n/` using the locale code, for example:

```text
i18n/es/
```

2. Copy the English base file:

```text
i18n/en/clip_paste.ftl
```

3. Translate every message into the target language.

## Updating an existing translation

When new strings are added, compare your locale file with the English file and make sure all keys are present.

Current message file format example:

```text
empty = Clipboard is empty
remove = Remove
pin = Pin
unpin = Unpin
delete-all = Clear History
search-placeholder = Search in clipboard history
no-results = No results found
```

## Translation rules

- Keep every message key exactly the same.
- Only translate the message values.
- Preserve placeholders and Fluent syntax if new strings introduce them.
- Keep the file name exactly `clip_paste.ftl`.
- Use UTF-8 text.
- Try to keep labels short enough for panel popup controls.

## Translation PR checklist

Please include in your PR:

- the locale code you added or updated
- whether it is a new translation or an update
- your preferred contributor tag for credits in this file / README-style docs
- confirmation that you checked your file against `i18n/en/clip_paste.ftl`

Suggested PR description:

```md
## Translation

- Locale: xx
- Type: new translation / update

## Notes

- Added or updated all keys from `i18n/en/clip_paste.ftl`
- Contributor tag for credits: @your-handle
```

## Translation PR expectations

- One locale per PR is preferred unless you are updating a shared string set across multiple languages.
- Keep translation PRs focused; avoid mixing code changes unless they are necessary for new strings.
- If you add a brand-new locale, also add yourself to the contributor list in this file.

## Related files

- `i18n.toml`
- `i18n/en/clip_paste.ftl`
- `src/i18n.rs`

## Updating the Description on Cosmic Store

The Cosmic Store displays the applet description and summaries using metadata from `resources/io.github.ajithakdev.clip-paste.metainfo.xml`.

### Where to edit
- The main applet description for the store is in the `<description>` block of the XML file. Each language uses an XML language tag, e.g.:
  - `<p>` (English, default)
  - `<p xml:lang="cs">` (Czech)
  - `<p xml:lang="it">` (Italian), etc.
- The short summary under `<summary>` at the top should also include translations, e.g. `<summary xml:lang="cs">...`.
- You can add or update language-specific blocks just as you would for application strings. Always use the appropriate `xml:lang` attribute for each language.

### How to update
1. Edit (or add) the `<summary>`, `<description>`, and any feature lists for your language by copying the English text and translating it inside the `<summary xml:lang="xx">` or `<p xml:lang="xx">` blocks.
2. For best results, keep non-translated versions (English) at the top and add/maintain language-specific tags directly beneath for each supported language.
3. For feature lists, you may add `<li xml:lang="xx">` inside `<ul>`, following the same structure as existing translations.
4. If adding new language translations, make sure to reflect your changes under both `<summary>` and inside the appropriate `<p xml:lang="xx">` sections in the `<description>`.
5. Only edit or add translations for languages you are confident in. Avoid machine translation unless reviewed by a human speaker.

### Validation and Submission
- After editing, ensure the XML remains valid and well-formed (no duplicate language blocks, every opening tag closed, only one default (English) block per section).
- Validate by building your package and checking how the description appears in the Cosmic Store (or review with Flatpak/Flathub tools for metadata validation).
- Commit and open a Pull Request with a clear note indicating which languages/descriptions were added or updated in the store metadata.

### When are changes reflected?
- Changes are picked up after new releases/bundles are created and published to the store. Make sure to merge to the main branch and include release notes for high-visibility updates.

### Example snippet
```xml
<summary>Clipboard history for COSMIC panel</summary>
<summary xml:lang="pl">Historia schowka dla panelu COSMIC</summary>
...
<description>
  <p>clip-paste is a COSMIC panel applet that keeps a history of recently copied text and images.</p>
  <p xml:lang="pl">clip-paste to aplet panelu COSMIC, który przechowuje historię ostatnio kopiowanych tekstów i obrazków.</p>
  ...
</description>
```

For further reference or details on translation/localization structure, see:
- [`resources/io.github.ajithakdev.clip-paste.metainfo.xml`](resources/io.github.ajithakdev.clip-paste.metainfo.xml)
- [Flatpak/Flathub AppStream documentation](https://docs.flatpak.org/en/latest/metadata.html)
