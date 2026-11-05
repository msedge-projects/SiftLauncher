# Gap matrix — Sift Launcher versus Modrinth App 0.22.0

## How to read this file

**This is a ledger, not a wish list.** Every row is one claim about the product, and a claim is only worth what its evidence is worth. The `status` column says *what is known*; the `ours (file:line)` and `gap` cells are where the proof has to be. A row that says only "missing", with nothing read and nothing captured behind it, is a row nobody has checked — and this project has already paid for that: six features were recorded here as unbuilt and every one of them turned out to be complete and merely unreached (the Files panel, the tab bar, the card body, the category filter, the instance card's context menu and the search field's clear button). Each was judged from a press that missed. The rule that came out of it is the one this file is now read by: **a press that does nothing is evidence about the press, not about the feature.**

| status | what it means | what the row has to carry |
|---|---|---|
| **DONE** | The reference's value or behaviour and ours are both stated and shown equal. **DONE means driven or tested, not asserted**: a run that produced a visible result, a test that holds the behaviour, or a measurement of both sides with its numbers. | the evidence, named: the drive, the test's name or the two measured values. |
| **VERIFIED** | The code that implements the behaviour **exists and was read**, at the `file:line` the row now cites. The row's original claim — "absent", "inert", "does nothing" — is false against today's source. | the `file:line` of the handler, the state or the draw call that does the work. |
| **OPEN** | The reference behaviour is stated and our code does not match it. **An OPEN row is a claim about the product, and it must cite a source read or a capture**: the `ours` cell names the file and line that stops short, or the `gap` cell names the capture and the measurement. An OPEN row that cites neither is not a finding yet. | a source read (`file:line`) or a capture with its numbers. |
| **OPEN-WITH-CAUSE** | Still open, and the cause is now named and cited: the code that exists at the `file:line` the row gives stops short in a specific way, or a named mechanism blocks it. | the cause and its `file:line`. A row in this state may not be re-described as "unbuilt" without reading the cause first. |
| **BLOCKED-SERVICE** | The element needs a hosted service this app has no client for: Modrinth auth/account (skins apply, sync, the signed-out gate), friends, or ads/promotions. The `⇒` at the end of the gap cell names which. No amount of drawing closes these; they need a decision about the service. | the `⇒` naming the service. |
| **RENDERER-RISK** | The reference effect is one a software rasteriser may not reproduce: a blur, a multi-layer shadow, an animated gradient, or antialiased tessellation. The `⇒` in the gap cell names the effect. These are OPEN work that also needs a renderer decision, and they are not counted as plain OPEN anywhere in this document. | the `⇒` naming the effect. |
| **NOT-IN-SOURCE** | 0.22.0 does not contain the thing at all — no shift-range selection, no file or group reorder, no world import/export, a `viewFilter` with no control, tokens the reference never reads, a card nothing imports. There is nothing to port and "missing" is the correct state. | the reference `file:line` that shows the absence, or the reason the brief lists a thing the app does not have. |

**DONE means driven or tested, and the row names the evidence.** Of the 90 DONE rows as counted on 2026-11-01, four rest on a capture measurement with its numbers in the gap cell (rows 1–3 and 8, all dated), 51 on a value compared on both sides — the sheet's line against our constant, both cited — 20 on the report's bare "none", 5 on a Renderer row whose risk is "nothing today", 6 on a match by absence, and **4 on nothing at all** (rows 284, 288, 289, 295 were flipped from OPEN to DONE in the working tree while this pass was being written, with no note and no evidence added — see §Counts). **No DONE row in this file names a test or a driven run.** That is the honest reading of the word as the table uses it, and it is the weakest part of this document: a token that matches is not a behaviour that works. A row that claims a behaviour rather than a value should not be believed as DONE until a drive or a test is written into it.

**OPEN rows were re-read on 2026-11-01.** 34 OPEN rows whose evidence was a press that never landed, or a drive that never happened, were checked against the source. 15 are now `VERIFIED` and 19 `OPEN-WITH-CAUSE`; the list, with the `file:line` read for each, is in §Verification ledger under "The rows that were called unbuilt and were merely unreached". Those 34 take OPEN from the 504 this file carried at HEAD (`dc3d330`) to 470. The count stands at **466 OPEN** as this paragraph was written, because four more rows left OPEN in the working tree while the pass ran — see §Counts, and see them named.

**Citations are the reports', and they were true when the report was written.** Checked on 2026-11-01, and every one of them resolves to a file in this tree at a line it contains, with two exceptions: `src/settings.rs`, which became `crates/sift-ui/src/pages/settings.rs`, and a `lib.rs` the crate has never had. **Resolving is not agreeing.** A citation that points inside a file can still point at code that has moved: `instance.rs:470-512` is cited by twelve Files-tab rows and now lands in `rename_entry` and the Content tab, not in the file list at all. That is why a row this file has re-read carries its own dated `file:line`, and why an unread row's citation is a lead rather than a fact.

**Every count in this file is re-derived with a script, and says when.** The command is in §Counts; a number here without a date is a number to re-run.

## What this document is

This is the merged work plan for making Sift Launcher identical to Modrinth App 0.22.0. It gathers every row of the ten audit reports in `.scratch/reports/` into one continuously numbered list: **625 rows**, made from 659 report rows, of which 34 were folded into 29 rows because a second report described the same element and the same defect. Every row came from a report; none was written to make the table look complete.

**What is the report's and what is this document's.** `element`, `reference value / behaviour`, `states & animation`, `ours` and `gap` are the report's own cells, copied verbatim; nothing here is a new finding. A merged row joins the other report's cells onto the first's, each addition tagged `‖ <report>.md row N — element "…"`, so the two statements stay separable and neither is paraphrased. `reference (file:line)` holds every `file:line` citation lifted unchanged out of the report's reference cell, in the order the report gives them, which is what makes a row checkable without opening the report. What this document adds is the numbering, the grouping into sections, the `status` column, and — for Renderer rows only, which that report did not prioritise — a priority.

**How a row reads.** `element` is `<area> — <element>` as the report names it. `reference (file:line)` is where 0.22.0 states the value; `reference value / behaviour` is that value with its brackets and units as written. `states & animation` is the report's states column — hover, press, focus, disabled, open/close, and any duration or easing, with its trigger; it reads `—` where the report has none, which is a fact about the report and not about the reference. `ours (file:line)` is our current code in the report's own words, so where it says `absent` or `none` no pointer exists because no code does. `gap` is the report's verdict, verbatim, and for a BLOCKED-SERVICE or RENDERER-RISK row a trailing sentence beginning `⇒` names the service or the effect, because that is the one thing a status column cannot say. `priority` is the report's own P0–P3, or `—` where the report marked the row a match or not a requirement. Where a row has been re-read after it was written, a dated sentence in this document's own voice follows the report's words — `RE-VERIFIED <date>: …` when a source read overturned a claim of absence, `CORRECTED on <date>: …` when the row's own verdict was wrong. A dated sentence is never the report speaking, and it always ends at a `file:line` that the reader can open.

**Paths.** Citations are copied exactly as the reports wrote them, and the reports do not share one base: the `page-*.md` reports and `motion.md` give full repository paths (`crates/sift-ui/src/…`, `apps/app-frontend/src/…`); `tokens.md` states in its own header that its `ours` paths are under `crates/sift-ui/src/`, and `plumbing.md` gives its call sites relative to the same directory; `file-management.md` abbreviates `af/` for `apps/app-frontend/src` and `ui/` for `packages/ui/src`; `interaction.md` abbreviates `APP`, `UI`, `STY` and `SRC` for those four trees. Nothing here rewrites a path, so the report named in a row is the way to resolve its shorthand.

**Where a row lives.** A row sits in the section of the element it names, not the section of the report it came from: the sidebar rows of the Home and Discover reports are in Shell, the Skins/Screenshots report supplies both of those sections, and the tokens report supplies Typography and half of Motion as well as Design tokens. Within a section, rows keep report order then report row number, so any run of rows can be read straight through in the report that made it.

**Statuses.** Every status is defined once, at the top of this file, in *How to read this file*. Read that table before reading a status here: it is what makes "VERIFIED" mean a source read and "DONE" mean a match that was shown.

**Priority.** P0 is a page-level behaviour missing or unusable; P1 visible behaviour differs; P2 geometry, typography and state drift; P3 a nit, a documented deviation, or a documented non-port. `—` marks a row the report called a match or not a requirement, so it carries no build order.

**The hole in this plan — corrected 2026-11-01.** This paragraph used to say that `page-servers-project-about.md` was **not in `.scratch/reports/`**. It is there: 57 rows (Servers 25, Project 23, About 9; 16 P0 / 21 P1 / 20 P2), summarised in §Verification ledger under "Servers, Project and About audited against source". What is still true is the consequence — **none of its rows has been folded into the tables below**, so §8 Servers/Hosting still carries the single row `interaction.md` happens to record, and §9 Project and §10 About carry none. That is an unfinished fold, not a missing audit, and it is the largest known hole in this document. As before, the sections are left in place with their real counts so the hole shows in the table of contents instead of being quietly closed by invention, and no row anywhere in this document was written to fill it.

## Table of contents

| § | section | rows | drawn from |
|---:|---|---:|---|
| 1 | Shell — title bar, rail, surface, sidebar | 13 | motion.md, page-discover.md, page-home.md, page-instance.md, tokens.md |
| 2 | Home | 42 | page-home.md |
| 3 | Discover | 69 | motion.md, page-discover.md |
| 4 | Instance | 61 | file-management.md, page-instance.md |
| 5 | Settings | 65 | interaction.md, motion.md, page-settings.md |
| 6 | Skins | 31 | file-management.md, interaction.md, page-skins-screenshots.md |
| 7 | Screenshots | 28 | file-management.md, motion.md, page-skins-screenshots.md |
| 8 | Servers / Hosting | 1 | interaction.md |
| 9 | Project | 23 | page-servers-project-about.md (its rows 26-48, folded in as 626-648) |
| 10 | About | 0 | **report exists (9 rows), rows not yet folded in — see "The hole in this plan"** |
| 11 | Design tokens | 78 | tokens.md |
| 12 | Typography | 27 | tokens.md |
| 13 | Motion | 66 | motion.md, tokens.md |
| 14 | Interaction / keyboard | 60 | interaction.md |
| 15 | File management | 56 | file-management.md |
| 16 | Renderer | 28 | plumbing.md |
| | **total** | **625** | ten reports, one of them not yet folded in |

## 1. Shell — title bar, rail, surface, sidebar

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 1 | Shell \| sidebar — section padding | apps/app-frontend/src/app/shell/sidebar/index.vue:108-122 | each section is `p-4 border-0 border-b-[1px] border-[--brand-gradient-border]`; the section rule sits at y 292 in the capture | static | crates/sift-ui/src/panel.rs:39 (`PAD = 16`), :52 (`SECTION_GAP = 28`) | CLOSED by measurement on 2026-10-31: padding is 16, media is 268 wide against the reference 267, and the vertical rhythm matches. What is still missing is the section rule at y 292 and the p-4 bottom padding | DONE | P1 |
| 2 | Shell \| sidebar — Getting started card | apps/app-frontend/src/features/onboarding/checklist.vue:92-94 | accordion card `rounded-2xl border border-t-0 border-surface-5 bg-button-bg p-3` inside `px-3 p-4`; the capture puts it at x 1157..1424, y 65..275, fill #2c3431, ring #3d4545 | accordion open/close 0.3s ease | crates/sift-ui/src/panel.rs:400-412 | CLOSED: the card exists at y 65..276 with fill #2a3432 and a ring, heading ink at y 84 in both. Residual: our card is 268 wide against 275 (the reference inset is px-3, so 12 not 16) and has no rule under its header at y 114 | DONE | P1 |
| 3 | Shell \| sidebar — Getting started row | checklist.vue:105-123 | `h-10 rounded-xl border border-button-border bg-button-bg px-4 py-2.5 shadow-[0_1px_0.5px_rgb(0_0_0/12%)]`, radio 20, label `font-medium leading-5`; capture: rows at y 128..166, 176..214, 224..262, fill #3a4341, ring #4b5253 | `transition-[filter]` | crates/sift-ui/src/panel.rs:104 (`ROW_H = 40`), :233, :470-500 | CLOSED: rows are 40 tall on a 48 pitch in both, fill #3b4442 against #3a4341, label 16 px, ring 24. Residual: the 12% drop shadow under each row is not drawn, and the label is `body` (400) where the reference is `font-medium` (500) | DONE | P1 |
| 4 | sidebar — news art | `packages/ui/src/components/content/NewsArticleCard.vue:26-29` | `aspect-video w-full rounded-xl border-[1px] border-button-border object-cover` (packages/ui/src/components/content/NewsArticleCard.vue:26-29) | whole card `hover:brightness-125`, `active:scale-[0.99]` (:23) | panel.rs:133 (`ART_H = 128`) | the art height is a constant rather than derived from the width, giving 1.97:1 where the reference shows 1.78:1 (267x150): our art is 22 px short | OPEN | P1 |
| 5 | sidebar — news type | `NewsArticleCard.vue:31-39` | title `text-base leading-tight`, summary `text-sm leading-tight text-primary`, date `text-sm text-secondary` (NewsArticleCard.vue:31-39) | hover:underline on the title (:31) | panel.rs:144-176 (14 / 12 / 12) | summary and date are one step small (12 against 14), which also pulls the card 76 px up the panel | OPEN | P1 |
| 6 | sidebar — upgrade control | `apps/app-frontend/src/features/news/news-sidebar.vue` (no line number in the report) | the reference shows an inline icon-plus-text link line (measured 232x30 at y606 in .scratch/cap/ref-home.png); which component renders it is UNKNOWN - evidence: the upgrade markup in apps/app-frontend/src/features/news/news-sidebar.vue | static | panel.rs:321 and its draw call | ours draws a full-width filled brand button (252x32 at y539) where the reference has a quiet inline link: a different element in the same slot ⇒ blocked on the promotions/ads service — the upgrade line is drawn by the reference's news/ad slot | BLOCKED-SERVICE | P0 |
| 7 | sidebar — scroll fades | `sidebar/index.vue:139-148`<br>`apps/app-frontend/src/app/shell/sidebar/index.vue:140`<br>`packages/assets/styles/variables.scss:154`<br>`packages/assets/styles/variables.scss:374` | two `pointer-events-none absolute inset-x-0 h-8` gradient overlays, the top one `rotate-180`, `transition-opacity duration-200 motion-reduce:transition-none`, shown while scrolled (sidebar/index.vue:139-148) ‖ motion.md row 16 — element "shell — sidebar scroll fades": `apps/app-frontend/src/app/shell/sidebar/index.vue:140` and `:144-148` opacity toggling with `duration-200` and `motion-reduce:transition-none` ‖ tokens.md row 49 — element "gradient — --brand-gradient-fade-out-color": linear-gradient(to bottom, rgba(213,235,224,0) 0%, #d0ece0 70%) packages/assets/styles/variables.scss:154; dark rgba(24,30,31,0) to #171d1e 80% packages/assets/styles/variables.scss:374 | 200ms opacity on scroll ‖ motion.md row 16: 200ms opacity, trigger = scroll position or `showAd` ‖ tokens.md row 49: opacity 0 to 100 over 200ms (apps/app-frontend/src/app/shell/sidebar/index.vue:140-147 transition-opacity duration-200), triggered by the scroll position | none ‖ motion.md row 16: none in `crates/sift-ui/src` (no sidebar scroll area) ‖ tokens.md row 49: absent | absent: the panel content ends at a hard edge instead of fading ‖ motion.md row 16: absent with the sidebar ‖ tokens.md row 49: not carried and not drawn: the sidebar's scroll fade is missing at both ends | OPEN | P2 |
| 8 | Shell \| sidebar — column | apps/app-frontend/src/app/shell/sidebar/index.vue:89, 106 | the 300-point right column, the brand gradient, and `border-l-[1px] border-[--brand-gradient-border]`: the capture shows x 1140 as #223328 at y 60 and #1e2a24 at y 880, one continuous pixel, against #1a2420 one pixel in | static | crates/sift-ui/src/panel.rs:375-390 | CORRECTED on 2026-10-31: this row was marked DONE and the code commented that the reference had no rule — both wrong. The rule is now painted after the gradient (so the gradient cannot cover it) and measures #212a27 at x 1140 against the background #192321 at x 1143. Residual: our rule is composed over `--color-bg` where the reference composes it over the gradient, so it is ~8 units less green | DONE | — |
| 9 | sidebar — nested tokens | `apps/app-frontend/src/app/shell/sidebar/index.vue:154-163`<br>`packages/assets/styles/variables.scss:372-373`<br>`apps/app-frontend/src/app/shell/sidebar/index.vue:155-163` | inside `.app-sidebar`, --color-button-bg and --surface-4 become rgba(255,255,255,0.08) and --surface-5 and --color-divider rgba(155,255,160,0.08) (apps/app-frontend/src/app/shell/sidebar/index.vue:154-163; packages/assets/styles/variables.scss:372-373) ‖ tokens.md row 50 — element "gradient — the sidebar's scoped tokens": .app-sidebar overrides --color-button-bg, --surface-4, --color-button-bg-hover, --surface-5, --color-divider and --color-divider-dark (apps/app-frontend/src/app/shell/sidebar/index.vue:155-163) | static ‖ tokens.md row 50: static scope, so one token has two values on screen at once | crates/sift-ui/src/panel.rs uses opaque `BUTTON_BG` and `DIVIDER` inside the panel ‖ tokens.md row 50: panel.rs:335-358 draws with the shell-wide theme only | panel controls keep opaque surface colours where the reference lets the gradient through ‖ tokens.md row 50: no scoped theme exists, so a control inside the sidebar cannot be painted with the value the reference gives it; a scoped copy of Theme passed into panel::draw is what would fix it | OPEN | P0 |
| 10 | sidebar — switch block | `packages/ui/src/layouts/shared/browse-tab/sidebar.vue:163-172` | `p-4` block with a 1 px bottom border in `--brand-gradient-border` (packages/ui/src/layouts/shared/browse-tab/sidebar.vue:163-172) | static | crates/sift-ui/src/panel.rs:456-474 a 26-tall row at the column's left, no 16-point padding, no rule | padding and dividing rule missing | OPEN | P1 |
| 11 | toggle — Hide already installed | `packages/ui/src/components/base/Toggle.vue:8-13` | `Toggle small`: `h-5 !w-[40px] rounded-full p-1` with a 1 px surface-5 border, `bg-brand` when on and `bg-button-bg` when off (packages/ui/src/components/base/Toggle.vue:8-13) | knob 12 → 14 on hover, 10 on active, translate-x 20 when on, 200 ms (Toggle.vue:16-30) | crates/sift-ui/src/panel.rs:475-499 a 34×18 track, 13-point knob, no border, no hover or active morph | 34×18 vs 40×20; knob 13 vs 12; border missing; no state animation | OPEN | P1 |
| 12 | toggle — label copy | `apps/app-frontend/src/pages/Browse.vue:1306-1314, 572-575`<br>`packages/ui/src/utils/common-messages.ts:459-462` | "Hide already installed" on the modpacks tab, "Hide content already installed" otherwise (apps/app-frontend/src/pages/Browse.vue:1306-1314, 572-575; packages/ui/src/utils/common-messages.ts:459-462) | per tab | crates/sift-ui/src/panel.rs:65 one constant | same string on every tab | OPEN | P2 |
| 13 | header — instance switcher | `apps/app-frontend/src/components/ui/QuickInstanceSwitcher.vue:293-330`<br>`apps/app-frontend/src/App.vue:2061-2069` | apps/app-frontend/src/components/ui/QuickInstanceSwitcher.vue:293-330 (rail list of running instances, 32px divider h-px w-8, spinner overlay, create NavButton), mounted at apps/app-frontend/src/App.vue:2061-2069 | TransitionGroup 'quick-instance'; installing items brightness .25 scale .85 (QuickInstanceSwitcher.vue:316) | crates/sift-ui/src/shell.rs:770-786 (create button only, below a 32px divider) | no running-instance list; note the reference switcher is in the rail, not the page header | OPEN | P2 |

## 2. Home

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 14 | hero — dot pattern, geometry | `apps/app-frontend/src/components/ui/WelcomeScreen.vue:88` | box 29.875rem tall by min(25.9375rem,80vw) wide, `-top-52`, `rounded-2xl`, `left-1/2 -translate-x-1/2` (apps/app-frontend/src/components/ui/WelcomeScreen.vue:88) | static, `aria-hidden` (WelcomeScreen.vue:89) | crates/sift-ui/src/pages/home.rs:616-645 | the field is drawn, but at one ink threshold the ref shows dots over 334x288 px while ours covers 199x200, and ref dots are 1 px tall against ours 2 px, so ours reads ~40% smaller and blurrier (.scratch/cap/ref-home.png vs ours-home5.png) | OPEN | P1 |
| 15 | hero — dot pattern, mask and pitch | `WelcomeScreen.vue:141-153` | pitch 0.5625rem, dot 25% of `--color-text-primary` at 0.5px radius, `opacity: 0.8`, mask `radial-gradient(ellipse at center, black 10%, transparent 68%)` (WelcomeScreen.vue:141-153) | static | home.rs:617-644 (fade at :628-630) | pitch matches (9 px dot rows in both captures) but the fade is normalised by half-width/half-height where CSS sizes the ellipse to the farthest corner, so our halo dies at ~68% of 207 px instead of the corner distance and the outer dot rings are missing ⇒ renderer risk: multi-stop radial-gradient alpha mask (per-dot falloff) | RENDERER-RISK | P1 |
| 16 | hero — mark | `WelcomeScreen.vue:91-93` | `size-[6.25rem]` box holding `modrinth-social-icon.png` at full size (WelcomeScreen.vue:91-93); the rounded green plate is the asset itself (apps/app-frontend/src/assets/welcome/modrinth-social-icon.png) | static | home.rs:418-430 | no 100x100 plate: `brand::paint_mark` draws an 80x91 glyph straight onto the page ground, so the hero loses its anchor (ref plate 553..651 by 313..411, corner radius ~24, sampled rgb(15,53,36); ours 562..641 by 316..406, radius 0) | OPEN | P0 |
| 17 | hero — title | `WelcomeScreen.vue:95`<br>`WelcomeScreen.vue:16` | `text-2xl font-semibold leading-8 text-contrast` (WelcomeScreen.vue:95), message `app.welcome-screen.title` = "Welcome to Modrinth" (WelcomeScreen.vue:16) | static | home.rs:434-440 (string at :437) | the string is rebranded to "Welcome to Sift Launcher" (deliberate product name, not a defect); the glyph band (443..460 ours, 444..460 ref) and 24/32 type match within 1 px | OPEN | P2 |
| 18 | hero — description width | `WelcomeScreen.vue:98` | `text-base leading-6 text-primary`, message "Ready to start playing?" (WelcomeScreen.vue:98, :19-20) | static | home.rs:442-448 | same string on the same band (482..496 ours, 483..497 ref), but ours sets 172 px wide against the ref 226; cause UNKNOWN (ours loads Inter 400/500/600, crates/sift-ui/src/fonts.rs:24-31, yet the reference reads wider per character at matching height) - evidence: computed letter-spacing and used font from a live 0.22.0 window | OPEN | P1 |
| 19 | hero — create button, box and pointer states | `packages/ui/src/components/base/buttons/ButtonFrame.vue:32`<br>`WelcomeScreen.vue:107` | `size="lg"`: `h-10 rounded-[14px] px-4 gap-2 text-base font-semibold [&>svg]:size-5` (packages/ui/src/components/base/buttons/ButtonFrame.vue:32) with `!shadow-none` (WelcomeScreen.vue:107) | `enabled:active:scale-[0.97]`, hover `brightness-[--hover-brightness]`, `transition-... duration-150 ease-out` (ButtonFrame.vue:18-21); `--hover-brightness: 1.25` in dark (packages/assets/styles/variables.scss:407) | home.rs:390-394, 451-464 | box matches (206x40 ours against 214x40 ref, top 524 against 525) but there is no hover brightening and no press scale: the page's one primary action gives no pointer feedback | OPEN | P1 |
| 20 | hero — create button, lit rim | `ButtonFrame.vue:151-164` | `::before` is a 1 px ring: `inset:0; padding:1px` masked content-box xor over `linear-gradient(180deg, rgba(255,255,255,0.3), rgba(255,255,255,0))` (ButtonFrame.vue:151-164) | static | home.rs:467-486 | ours paints a white 30%-to-0 mesh across the top HALF of the button (home.rs:473-486) instead of a 1-pixel rim, so the button reads two-tone where the reference reads flat with a lit top edge | OPEN | P0 |
| 21 | hero — key hint | `WelcomeScreen.vue:114-124` | `span gap-1 text-sm leading-5 text-secondary` around `kbd h-5 min-w-5 rounded-md border border-surface-5 bg-button-bg px-1 text-xs leading-4 text-primary` (WelcomeScreen.vue:114-124); string "Press <shortcut>N</shortcut> to quick create an instance" (:28) | static | home.rs:508-542 (keycap at :528-535) | line box matches (242x20 against 248x20) but the keycap is radius 6 where `rounded-md` is 12 pt (variables.scss:203), so the cap reads square against the reference's pill | OPEN | P2 |
| 22 | hero — quick-create key | `WelcomeScreen.vue:50-68`<br>`APP/components/ui/WelcomeScreen.vue:50-68` | window `keydown`: key `n`, ignored when `event.repeat`, meta, ctrl, alt, contentEditable, or INPUT/TEXTAREA/SELECT, and only when online (WelcomeScreen.vue:50-68, listener :70-74) ‖ interaction.md row 1 — element "keyboard — `N` = quick-create an instance": `APP/components/ui/WelcomeScreen.vue:50-68` rejects `repeat`, `meta`, `ctrl`, `alt`, `INPUT`/`TEXTAREA`/`SELECT` targets and contenteditable, `preventDefault`s, and does nothing offline; registered on `window` at `:73`, mounted only with the welcome screen | trigger: keydown n ‖ interaction.md row 1: — (this report has no states column) | home.rs:597-616 (`quick_key`) | no repeat, modifier or field guard and no online gate, so Ctrl+N and an "n" typed into any future text field create an instance ‖ interaction.md row 1: no modifier test, no repeat test, no text-target test, no offline gate; the page is still drawn under both modals (`SRC/shell.rs:380` runs before `about::draw`/`settings::draw` at `:405-406`), so `n` creates an instance behind an open modal; `Ctrl+N`/`Shift+N` also fire . PARTLY CLOSED 2026-11-02: the guards the reference states for the key itself are in — no modifier, no `event.repeat`, and nothing while a widget holds the keyboard (`memory.focused()`), each pinned by a headless test that draws the hero with one synthetic key event (`the_quick_create_key_ignores_modifiers_repeats_and_a_held_keyboard`). Still open: the offline gate (`WelcomeScreen.vue:71-72` — there is no offline concept in this build) and the fact that the page is drawn under an open modal, which is `shell.rs:380`'s ordering and not the page's to fix | OPEN | P0 |
| 23 | hero — offline state | `WelcomeScreen.vue:108`<br>`WelcomeScreen.vue:40-48, 71-72`<br>`ButtonFrame.vue:24` | both buttons carry `:disabled="offline"` (WelcomeScreen.vue:108, :132), driven by window `offline`/`online` (WelcomeScreen.vue:40-48, 71-72); `disabled:opacity-50 disabled:cursor-not-allowed` (ButtonFrame.vue:24) | trigger: offline/online event | home.rs:594-606 | no offline concept: both buttons stay enabled and the quick-create key still fires (home.rs:597) | OPEN | P2 |
| 24 | hero — import glyph | `WelcomeScreen.vue:133` | `ImportIcon` (WelcomeScreen.vue:133) | static | home.rs:572-580 | draws Glyph::BoxImport, an isometric cube, where the reference shows an import/tray arrow | OPEN | P1 |
| 25 | footer — import button surface | `ButtonFrame.vue:45, 135-140` | default `type="base"`: `bg-surface-4 text-contrast` plus `box-shadow: inset 0 0 0 1px var(--surface-5), 0 1px 1px rgba(0,0,0,0.12)` (ButtonFrame.vue:45, 135-140) | hover brightness 1.25, active scale .97, 150ms ease-out (ButtonFrame.vue:18-21) | home.rs:552-588 | box matches (220x40 against 222x40, y836 against 837) and the inset ring is drawn, but the 1 px drop shadow is missing and there is no hover or active feedback | OPEN | P2 |
| 26 | state — which screen the populated page shows | `apps/app-frontend/src/pages/Index.vue:93-105`<br>`apps/app-frontend/src/components/ui/library/instance-group/index.vue:123-142` | populated Home is `RecentWorldsList` over `LibrarySection` (apps/app-frontend/src/pages/Index.vue:93-105), and the library draws instance cards in a responsive grid (apps/app-frontend/src/components/ui/library/instance-group/index.vue:123-142) | static | home.rs:342-355 paints `list()`, a column of 96 px horizontal rows (home.rs:696-703); crates/sift-ui/src/pages/library.rs is never called from crates/sift-ui/src/shell.rs | the whole populated state is a bespoke row list and the ported card grid is dead code, so nothing the brief asks about for cards (grid, size, hover, context menu) is on screen | OPEN | P0 |
| 27 | state — jump-in position | `Index.vue:96-103` | `RecentWorldsList` renders ABOVE `LibrarySection` inside `flex flex-col gap-3 p-6` (Index.vue:96-103) | static | home.rs:797-860 | ours draws the "Jump in" block BELOW the instance list, inverting the populated page's stacking order | OPEN | P0 |
| 28 | populated — library heading | `apps/app-frontend/src/components/ui/library/index.vue:268-270`<br>`Index.vue:96` | `h2.m-0.text-2xl.font-semibold.text-contrast` "Library" (apps/app-frontend/src/components/ui/library/index.vue:268-270) inside `flex flex-col gap-3 pb-16 min-h-[500px]` (:267), itself inside the page's `p-6` (Index.vue:96) | static | crates/sift-ui/src/pages/library.rs:332 | `Page::rect` already returns the padded column starting at x 88 (crates/sift-ui/src/pages/mod.rs:70-83), so adding PAGE_PADDING puts the heading and the grid at 112: 24 px right of the reference's 88 | OPEN | P0 |
| 29 | populated — toolbar row 1 | `apps/app-frontend/src/components/ui/library/library-toolbar/index.vue:30-48` | search `Input min-w-[16rem] flex-1` + "New group" `Button` + "New instance" `type="colored" color="brand"` `Button`, in `flex flex-wrap gap-2` (apps/app-frontend/src/components/ui/library/library-toolbar/index.vue:30-48) | hover/open per Button and Input | library.rs:369-397 | entirely absent: no search field, no "New group", no "New instance", so the populated page loses its primary creation affordance RE-VERIFIED 2026-11-01: the search field this row calls absent is in the library heading row — it takes the keyboard, draws the query and filters the list (home.rs:672-712, the filter at :713-722) — while New group and New instance are still absent. . RE-VERIFIED BY DRIVING 2026-11-02: the field filters (6 rows to 3 on "inst"), draws the reference's no-results line, and now gives the keyboard back on a press anywhere else on the page (home.rs:868-875) — without that, one click on the field took every later keystroke on the page. "New group" (`button.new-group`) and "New instance" (`button.new-instance`, `type="colored" color="brand"`) are still absent, and the field still sits in the heading row instead of the toolbar row they belong to | OPEN-WITH-CAUSE | P0 |
| 30 | populated — toolbar row 2 | `apps/app-frontend/src/components/ui/library/library-toolbar/sort-menu.vue:62-90`<br>`library-toolbar/index.vue:49-53`<br>`.../filter-menu.vue:59-69` | `SortMenu` (sort and group-by `Combobox`es, apps/app-frontend/src/components/ui/library/library-toolbar/sort-menu.vue:62-90), then `mx-2 h-6 w-px bg-surface-5` divider, then `FilterMenu` `DropdownFilterBar` (library-toolbar/index.vue:49-53; .../filter-menu.vue:59-69) | Combobox hover/focus/open | library.rs:379-397, 463-487 | the divider is drawn BETWEEN the two comboboxes (library.rs:380-392) though the source keeps both inside SortMenu and puts the divider after them; the filter bar is missing; the two-segment view toggle (library.rs:463-487) is invented, compact mode is a setting (instance-group/index.vue:57) | OPEN | P0 |
| 31 | populated — count pill | `instance-group/index.vue:571-576` | the count appears only as the group header's `TagItem` (instance-group/index.vue:571-576), and that header is hidden when the ungrouped group is the only one (:335-337, :372) | static | library.rs:342-367 | ours prints a count pill beside the page heading, an element the reference never shows there | OPEN | P1 |
| 32 | populated — group header | `instance-group/index.vue:517-553` | `h-10 flex w-full items-center gap-2 border-b border-surface-5`, `DropdownIcon size-5 transition-all duration-300` rotating 180, name `text-base font-semibold` (instance-group/index.vue:517-553) | accordion open/close `grid-template-rows 0.3s ease-in-out` (packages/ui/src/components/base/Accordion.vue:128) | none | no group header, no count tag, no collapse | OPEN | P1 |
| 33 | grid — grid width feeding the column count | `instance-group/index.vue:123-129` | `cardWidth = (gridWidth - gap*(n-1))/n` with `minWidth` 10rem at or above 1280 (instance-group/index.vue:123-129) | static | library.rs:290, 527-536 | same arithmetic over a 980 px grid instead of the reference's 1028, so five cards of 186.4 px stand where the reference's five are 196 px | OPEN | P0 |
| 34 | grid — card height | `instance-group/index.vue:130-134` | `cardHeight = cardWidth + remSize * 3.375` (instance-group/index.vue:130-134) | static | library.rs:539-541 (the extra row at :182) | ours adds a further 18-point PLAYED_LINE, so a 1440 card is 258.4 tall against the reference's 240.4 | OPEN | P0 |
| 35 | card — hover and press | `apps/app-frontend/src/components/ui/library/instance-group/instance-card.vue:266`<br>`instance-card-view.vue:41`<br>`apps/app-frontend/src/components/ui/library/instance-group/instance-card-view.vue:41`<br>`apps/app-frontend/src/components/ui/library/instance-group/instance-card.vue:270` | `hover:brightness-110` with `transition-[background-color,border-color,filter]` (apps/app-frontend/src/components/ui/library/instance-group/instance-card.vue:266; instance-card-view.vue:41); `scale-[0.95]` on primary pointerdown (:270); `opacity-50` while part of a drag (:269) ‖ motion.md row 61 — element "cards — card hover": `apps/app-frontend/src/components/ui/library/instance-group/instance-card.vue:266` `hover:brightness-110`; `apps/app-frontend/src/components/ui/library/instance-group/instance-card-view.vue:41` `transition-[background-color,border-color,filter]` ‖ motion.md row 62 — element "cards — card press": `apps/app-frontend/src/components/ui/library/instance-group/instance-card.vue:270` `scale-[0.95]` while the primary pointer is down (`:290`) | trigger: hover, pointerdown, drag ‖ motion.md row 61: brightness only: 0.22.0 has no hover lift, translate or shadow change (not stated in source) ‖ motion.md row 62: 0.95 shrink for the length of the press, ramped by the view's filter transition | none ‖ motion.md row 61: `crates/sift-ui/src/pages/home.rs:762-781` (kebab only) ‖ motion.md row 62: none in `crates/sift-ui/src` | no hover brightening and no press scale, on either the row list or the grid card ‖ motion.md row 61: card has no hover feedback at all; the 110% brightness ramp is missing ‖ motion.md row 62: absent | OPEN | P1 |
| 36 | card — focus and activation | `instance-card.vue:266, 278-288, 156-170` | `role="button" tabindex="0"`, `focus-visible:!outline-2 -outline-offset-2`, Enter opens the instance, Space toggles selection (instance-card.vue:266, 278-288, 156-170) | keydown Enter/Space | home.rs:726-749 | the card body is not a keyboard target and its press is inert by design (home.rs:737-739), where the reference's whole card opens the instance RE-VERIFIED 2026-11-01: the card body's press exists and opens the instance's page (home.rs:776-780, emitting `Action::Edit`); the keyboard half of the row — a focusable card — is still missing. | OPEN-WITH-CAUSE | P1 |
| 37 | card — icon tile | `instance-card-view.vue:50-64`<br>`packages/ui/src/components/base/Avatar.vue:22-48` | `aspect-square min-w-full rounded-2xl` Avatar `size="100%"` `tint-by="instance.id"` `pad-transparent-corners` (instance-card-view.vue:50-64); the iconless fallback is a cuboid svg tinted `oklch(50% 75% <hue>)` (packages/ui/src/components/base/Avatar.vue:22-48) | lazy load (instance-card-view.vue:59) | library.rs:564-586 | a flat surface-4 plate with the name's first letter: no icon, no id tint; the rendered fallback at card size is UNKNOWN because no populated capture exists - evidence: a capture with one iconless instance | OPEN | P1 |
| 38 | card — loader line | `instance-card-view.vue:84-90` | `{{ instance.loader }} {{ instance.game_version }}` at `text-sm font-medium capitalize leading-[18px] text-primary` (instance-card-view.vue:84-90) | static | library.rs:617-631 | 13 px instead of 14 and no `capitalize`, so ours prints "neoforge 1.21" where the reference prints "NeoForge 1.21" | OPEN | P1 |
| 39 | card — last-played line | `instance-card-view.vue:73-91`<br>`apps/app-frontend/src/components/ui/world/InstanceItem.vue:40` | the card has two text lines only, name and loader/version (instance-card-view.vue:73-91); relative time belongs to the jump-in row (apps/app-frontend/src/components/ui/world/InstanceItem.vue:40) | static | library.rs:637-648 | ours invents a third line inside the card ("Played ..." / "Never played") and reserves 18 px for it, which is also why the card is too tall | OPEN | P0 |
| 40 | card — play button | `packages/ui/src/components/base/buttons/IconButton.vue:29`<br>`instance-card-view.vue:66-71`<br>`instance-card.vue:337, 354`<br>`apps/app-frontend/src/components/ui/Instance.vue:278` | `IconButton type="colored" color="brand" size="lg"` (circular by default, packages/ui/src/components/base/buttons/IconButton.vue:29) in a `size-12` slot at `bottom-1.5 right-1.5` of the tile (instance-card-view.vue:66-71), class `origin-bottom scale-75 opacity-0 transition-opacity group-hover/card:scale-100 group-hover/card:opacity-100` (instance-card.vue:337, 354) ‖ motion.md row 63 — element "cards — card play overlay": `apps/app-frontend/src/components/ui/Instance.vue:278` and `:290` `transition-all scale-75 group-hover:scale-100 ... opacity-0 group-hover:opacity-100 origin-bottom` | trigger: card hover; opacity through `transition-opacity` (duration not stated in source), the scale snaps because no transform transition is declared ‖ motion.md row 63: grows from 0.75 and fades in on hover; duration not stated in source | library.rs:588-597, home.rs:726-736 ‖ motion.md row 63: `crates/sift-ui/src/pages/library.rs:66-67` (play button drawn at rest) | drawn at rest on every card, so the page shows five green play buttons the reference hides until hover; the 40 px brand circle itself matches ‖ motion.md row 63: overlay is always visible instead of being revealed | OPEN | P0 |
| 41 | card — playing and installing indicator | `instance-card.vue:22, 84-101, 292-305, 312-323` | playing shows a red `StopCircle` IconButton; the overlay appears only after `LOADING_INDICATOR_DELAY_MS = 100`, then `absolute inset-0 bg-surface-1 opacity-30` plus `SpinnerIcon animate-spin` at `size-[30%]` (instance-card.vue:22, 84-101, 292-305, 312-323) | trigger: install or launch, 100 ms delay before showing | none | no stop button, no spinner, no delayed 30%-dim overlay: a launching instance looks identical to an idle one | OPEN | P1 |
| 42 | card — selection affordance | `instance-card.vue:363-388` | hover reveals a 24 px circle at `right-2 top-1.5` inside a `size-[50px]` hit area, `border-2 border-solid border-primary`, `opacity-0 transition-opacity duration-200 ease-out group-hover/card:opacity-100`; selected fills `bg-contrast` and draws `CheckIcon size-4 invert [stroke-width:3]` (instance-card.vue:363-388) | 200ms ease-out opacity on hover; selected state | none | no selection circle and no shift-range selection (apps/app-frontend/src/components/ui/library/index.vue:190-236), so the selection action bar has nothing to act on | OPEN | P1 |
| 43 | card — context menu trigger and anchor | `instance-group/index.vue:645-648`<br>`packages/ui/src/components/base/buttons/ContextMenu.vue:32-33, 54-56`<br>`APP/components/ui/library/instance-group/index.vue:645-647`<br>`APP/components/ui/library/use-library.ts:1279-1378`<br>`APP/components/ui/library/index.vue:400` | right-click anywhere on the card: `@contextmenu.prevent.stop` (instance-group/index.vue:645-648) opens `ContextMenu` at the pointer with `placement 'bottom-start'` and `distance 0` (packages/ui/src/components/base/buttons/ContextMenu.vue:32-33, 54-56) ‖ interaction.md row 18 — element "context menu — instance card, right-click": `APP/components/ui/library/instance-group/index.vue:645-647` (`@contextmenu.prevent.stop`), handler `:289-294`, items built at `APP/components/ui/library/use-library.ts:1279-1378`; menu label `APP/components/ui/library/index.vue:400` | trigger: contextmenu ‖ interaction.md row 18: — (this report has no states column) | home.rs:754-793 (kebab, left click) and home.rs:89-91 (distance 8) ‖ interaction.md row 18: only the kebab opens a menu (`SRC/pages/home.rs:754-793`); a right-click on the card or row falls through to nothing (`SRC/pages/home.rs:696-700` registers no secondary-button sense) | wrong gesture and wrong anchor: ours needs a left click on a 28 px kebab and hangs the panel 8 px off it, where the reference opens at the cursor with no offset; that kebab belongs to the jump-in row (`MoreVerticalIcon`, InstanceItem.vue:6, 79-82), not to a library card ‖ interaction.md row 18: right-click does not open the menu on any surface | OPEN | P0 |
| 44 | card — menu contents | `apps/app-frontend/src/components/ui/library/use-library.ts:1279-1378`<br>`use-library.ts:1280-1287`<br>`use-library.ts:135-186` | up to twelve rows with dividers: Stop (tone red, while playing), Play (tone brand), favourites toggle, Add content, View instance (EyeIcon), Duplicate (disabled unless installed), Open folder, Copy path, Edit icon submenu, Remove from group, Delete (tone red, `hoverFilledOnly`) (apps/app-frontend/src/components/ui/library/use-library.ts:1279-1378; labels :136-186) ‖ interaction.md row 19 — element "context menu — instance menu items, icons and strings": 11 rows + 2 dividers: `Stop` `StopCircleIcon` red, shown while playing (`use-library.ts:1280-1287`); `Play` `PlayIcon` brand, disabled while installing (`:1288-1296`); `Add/Remove from favorites` `StarIcon` (`:1297-1312`); divider (`:1313`); `Add content` `PlusIcon` (`:1314-1320`); divider (`:1321`); `View instance` `EyeIcon` (`:1322-1328`); `Duplicate instance` `ClipboardCopyIcon`, disabled unless installed (`:1329-1335`); `Open folder` `FolderOpenIcon` (`:1336-1341`); `Copy path` `ClipboardCopyIcon` (`:1342-1347`); `Edit icon` submenu `EditIcon` with 3 children (`:1348-1354`, `:1234-1262`); `Remove from group` `MinusIcon`, conditional (`:1355-1365`); divider (`:1366`); `Delete` `TrashIcon` red, `hoverFilledOnly` (`:1367-1377`). Strings: `use-library.ts:135-186` (`app.library.instance.action.play` … `:136`) | disabled rows `cursor-not-allowed opacity-50` (packages/ui/src/components/base/buttons/button-menu/button-menu.ts:17) ‖ interaction.md row 19: — (this report has no states column) | home.rs:216-282 ‖ interaction.md row 19: `SRC/pages/home.rs:200-221` five rows — Play/Edit/Duplicate/Open folder/Delete — with glyphs `:259-263`, keys `:247-251`, and no dividers, no disabled state, no submenu; popover drawn `SRC/pages/home.rs:884-975` | five rows, no dividers, no brand tone on Play, no disabled rules; our "Edit" row is the source's "View instance" with an Eye glyph (use-library.ts:1322-1328) ‖ interaction.md row 19: 8 of 11 actions missing (Stop, favourites, Add content, View instance, Copy path, Edit icon submenu, Remove from group); no dividers; no per-row `shown`/`disabled` logic; Play is labelled from `app.instance.action.play` (`SRC/pages/home.rs:247`) where the library menu uses `app.library.instance.action.play` (`use-library.ts:136`) . RE-VERIFIED BY DRIVING 2026-11-02: the menu still has five of the reference's eleven rows, but `Delete` now asks first — the press opens `ConfirmDeleteInstanceModal` and `Action::Delete` leaves the page only on the dialog's own button (home.rs: `press_menu_row`, `resolve_confirm`, `confirm_dialog`); Cancel on a throwaway instance left its folder on disk and the dialog's delete removed it (`.scratch/logs/home-drive.log`) | OPEN | P0 |
| 45 | card — menu row metrics | `button-menu.ts:14-18` | `min-h-10 gap-2 rounded-[10px] px-3 py-2 text-base font-semibold text-contrast hover:bg-surface-4`, icons `size-5 text-primary` (button-menu.ts:14-18) | hover and focus-visible fill | home.rs:926-969 | row box, radius and padding match; the leading glyph is 16 px against the source's 20 | OPEN | P2 |
| 46 | card — menu keyboard | `button-menu.ts:102-128, 179-193, 29`<br>`UI/components/base/buttons/button-menu/button-menu.ts:102-131`<br>`UI/components/base/buttons/button-menu/button-menu.ts:29, 149-177` | ArrowUp/ArrowDown/Home/End move focus, Escape closes, Tab closes, single-letter typeahead with a 500 ms reset (button-menu.ts:102-128, 179-193, 29) ‖ interaction.md row 7 — element "keyboard — arrow-key navigation inside a menu": `UI/components/base/buttons/button-menu/button-menu.ts:102-131` (ArrowDown/Up wrap, `Home`/`End`) ‖ interaction.md row 8 — element "keyboard — type-ahead inside a menu": `UI/components/base/buttons/button-menu/button-menu.ts:29, 149-177` | keydown; 500 ms typeahead reset ‖ interaction.md row 7: — (this report has no states column) ‖ interaction.md row 8: — (this report has no states column) | home.rs:441-535 (`menu_press`), :1300-1350 (the events it reads), :1412-1420 (the shared highlight) | none: ArrowUp/ArrowDown wrap, Home and End jump, Enter activates the focused row, Escape and Tab close, and a printable character is the type-ahead with the 500 ms reset. The pointer and the keyboard share one highlight fill. Two tests cover the rules (`the_menu_moves_and_activates_from_the_keyboard`, `the_type_ahead_jumps_to_a_label_and_resets_after_half_a_second`); the drawing is the same `bg-surface-4` the hover already used. CLOSED 2026-11-02 — the row's own interaction.md citation asks for exactly these seven keys | DONE | P1 |
| 47 | jump-in — section heading | `apps/app-frontend/src/components/ui/world/RecentWorldsList.vue:413-421` | Accordion whose title is `text-2xl font-semibold leading-none text-contrast mr-1` on a button `mt-1 mb-3` (apps/app-frontend/src/components/ui/world/RecentWorldsList.vue:413-421), message "Jump in" (:47) | collapsible accordion | home.rs:803-811 | drawn at 16 px semibold with no accordion, one type step below the reference's 24 px section title | OPEN | P1 |
| 48 | jump-in — rows | `RecentWorldsList.vue:67, 426, 532-540` | `.jump-back-in-item { height: 5rem }` in a `repeat(auto-fill, minmax(670px,1fr))` grid, at most `MAX_JUMP_BACK_IN = 5` items (RecentWorldsList.vue:67, 426, 532-540) | enter/leave `opacity, transform, height 0.25s ease`, reduced-motion override (:542-569) | home.rs:113-140 | 6 rows of 32 px, worlds only, no instance rows: a quarter of the reference's row height and a different row model | OPEN | P0 |
| 49 | jump-in — resize handle | `RecentWorldsList.vue:69, 497-527` | `h-6 w-1/3 mx-auto cursor-ns-resize` with two `h-px w-6` bars `bg-secondary` turning `bg-red` on overdrag; `ITEM_DRAG_DISTANCE = 92`; arrow/Home/End keys (RecentWorldsList.vue:69, 497-527) | drag and keyboard; overdrag flash 500 ms (:119-126) | none | no way to change how many jump-in items show | OPEN | P2 |
| 50 | jump-in — loading | `RecentWorldsList.vue:423-425` | `LoaderCircleIcon mx-auto size-8 animate-spin text-contrast` inside `text-center py-4` (RecentWorldsList.vue:423-425) | infinite spin while the recent-worlds query loads | none | absent; worlds are read synchronously instead (home.rs:315-329) | OPEN | P2 |
| 51 | loading — library gate | `apps/app-frontend/src/components/ui/library/index.vue:285-286, 278-284` | the grid area renders nothing until `libraryGroupsLoaded`, then the section fades in over 200 ms ease-out (apps/app-frontend/src/components/ui/library/index.vue:285-286, 278-284) | 200ms ease-out enter | library.rs:296-320 paints cards immediately | no loading gate; whether 0.22.0 ever paints a placeholder on this route is UNKNOWN - evidence: a capture taken during the first library load | OPEN | P2 |
| 52 | loading — skeleton component | `packages/ui/src/components/base/LoadingIndicator.vue:40-112`<br>`packages/ui/src/stories/base/LoadingIndicator.stories.ts:3` | no Skeleton component exists anywhere in packages/ui; the only shimmer in the design system is `LoadingIndicator`: 4rem plates at `--radius-lg`, opacity 0.25 on `--color-raised-bg`, `@keyframes pop` and `shimmer` both `4s ease-in-out infinite` with 0/0.3/0.6 s stagger (packages/ui/src/components/base/LoadingIndicator.vue:40-112), referenced only from its story (packages/ui/src/stories/base/LoadingIndicator.stories.ts:3) | 4s ease-in-out infinite | none | nothing to match on Home or Library: do not add a skeleton here, the reference's own loading state on this route is a spinner (row 37) plus an empty grid (row 38) | OPEN | P2 |
| 53 | empty — empty group message | `instance-group/index.vue:279-282, 651-657` | "Drag and drop to add instances." at `col-span-full m-0 pt-1 pl-0.5 text-base font-base text-secondary opacity-80` (instance-group/index.vue:279-282, 651-657) | static | none | absent; an empty group draws a bare grid | OPEN | P2 |
| 54 | empty — no search results | `library/index.vue:31-34, 272-277` | "No instances match your search." at `text-base text-primary` (library/index.vue:31-34, 272-277) | static | home.rs:929-937 (`list`, drawn when the filter empties the list) | none: the string is the reference's, `text-base text-primary`, at the first row's own top (content y 120). RE-VERIFIED BY DRIVING 2026-11-02: with "zzz" in the library field the probe printed `rows=0` and the line drew at content (88, 120); 751 near-white pixels in the band screen 145..159 (`.scratch/cap/drive-search-empty.png`) | DONE | P2 |
| 55 | empty — page background context menu | `Index.vue:70-88, 93-97` | right-click on `[data-library-page-background]` opens one row "New instance" with `PlusIcon` (Index.vue:70-88, 93-97) | trigger: contextmenu on the page ground | none | absent | OPEN | P2 |

## 3. Discover

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 56 | frame — page content inset | `apps/app-frontend/src/pages/Browse.vue:1333`<br>`apps/app-frontend/src/App.vue:2150-2158` | `p-6` on the route container, so content starts at y = 48 + 24 = 72 and x = 88 (apps/app-frontend/src/pages/Browse.vue:1333; apps/app-frontend/src/App.vue:2150-2158) | static (no state in source) | crates/sift-ui/src/shell.rs:1204-1207 draws the strip at y = g::TAB_CONTAINER_TOP = 48 (crates/sift-ui/src/geometry.rs:104) and x = content.left − 8 = 56 | no 24-point inset: the strip is flush on the surface's corner, 24 too high and 32 too far left | OPEN | P0 |
| 57 | frame — gap between page children | `apps/app-frontend/src/pages/Browse.vue:1333` | 8 (`gap-2`, apps/app-frontend/src/pages/Browse.vue:1333) | static | crates/sift-ui/src/shell.rs:1265 and 1509 use 12 | 12 vs 8 between the strip, the field, the toolbar row and the list | OPEN | P0 |
| 58 | frame — column width | `apps/app-frontend/src/App.vue:2158, 2349-2355`<br>`apps/app-frontend/src/app/shell/sidebar/index.vue:89`<br>`packages/assets/styles/defaults.scss:139-142` | 1440 − 64 rail − 300 sidebar − 12 scrollbar gutter = 1064, page inner 1016 (apps/app-frontend/src/App.vue:2158, 2349-2355; apps/app-frontend/src/app/shell/sidebar/index.vue:89; packages/assets/styles/defaults.scss:139-142) | static; `scrollbar-gutter: stable` (App.vue:2354) | crates/sift-ui/src/shell.rs:1194-1197, `PANEL_LEFT − CONTENT_LEFT − 16` = 1060 | 4 points narrow; the gutter is hardcoded 16 where the sheet states 12 | OPEN | P2 |
| 59 | frame — page scrolling | `apps/app-frontend/src/App.vue:2349-2355` | one scroll container for the whole page, `.app-viewport { overflow: auto }` (apps/app-frontend/src/App.vue:2349-2355) | wheel/touch; scrollbar always reserved | crates/sift-ui/src/shell.rs:1508-1521 only the list scrolls; strip, field, toolbar and pager stay pinned | the reference's toolbar scrolls out of view with the page | OPEN | P1 |
| 60 | frame — breadcrumb label | `apps/app-frontend/src/pages/Browse.vue:87-95, 104-109` | "Discover {projectType}" → "Discover modpacks" (apps/app-frontend/src/pages/Browse.vue:87-95, 104-109) | per route | crates/sift-ui/src/shell.rs:56 `Rail::Discover.title()` = "Discover modpacks" | none; where the title bar prints it is UNKNOWN without a Discover capture | DONE | — |
| 61 | tabs — strip container | `packages/ui/src/components/base/NavTabs.vue:9-13` | `w-fit rounded-full bg-bg-raised p-1` plus `card-shadow border border-surface-4` → 44 tall, radius 22, hugging the six pills (packages/ui/src/components/base/NavTabs.vue:9-13) | static | crates/sift-ui/src/shell.rs:1204-1208 spans content width + 8, height 52 (crates/sift-ui/src/geometry.rs:107), radius 12, no border or shadow | width (~1068 vs ~566 for six pills), height 52 vs 44, radius 12 vs 22, border and shadow missing | OPEN | P0 |
| 62 | tabs — pill box | `packages/ui/src/components/base/NavTabs.vue:9, 24, 35` | `px-4 py-2` at `text-sm`/20-line → 36 tall `rounded-full`, pills adjacent with no gap, label `font-bold` (packages/ui/src/components/base/NavTabs.vue:9, 24, 35) | hover: `.button-animation` opacity .5s, filter .2s, transform .05s (packages/assets/styles/defaults.scss:97-107) | crates/sift-ui/src/shell.rs:1231-1234, 1259 — 36 tall and text+32 wide, but x = pill.right() + 8 and semibold(14) | 8 points of invented gap between pills; weight 600 vs 700 | OPEN | P0 |
| 63 | tabs — inactive label ink | `packages/ui/src/components/base/NavTabs.vue:170-175`<br>`packages/assets/styles/variables.scss:338` | `text-contrast` = #ffffff (packages/ui/src/components/base/NavTabs.vue:170-175; packages/assets/styles/variables.scss:338) | static | crates/sift-ui/src/shell.rs:1222-1230 lays the label out in `theme.BASE` (#b0bac5, crates/sift-ui/src/theme.rs:494) | base instead of contrast | OPEN | P0 |
| 64 | tabs — active fill | `packages/ui/src/components/base/NavTabs.vue:63-72`<br>`packages/assets/styles/variables.scss:376` | slider `bg-button-bgSelected` = rgba(27,217,106,0.25) (packages/ui/src/components/base/NavTabs.vue:63-72; packages/assets/styles/variables.scss:376) | static per route | crates/sift-ui/src/shell.rs:1236-1238 `BUTTON_BG_SELECTED` | none | DONE | — |
| 65 | tabs — tab change animation | `packages/ui/src/components/base/NavTabs.vue:254-267, 367-381`<br>`packages/ui/src/components/base/NavTabs.vue:367-381` | slider left/right/top/bottom 80 ms cubic-bezier(0.4,0,0.2,1) with a 175 ms stagger on the trailing edges; label colour 100 ms plus a 100 ms delay when the index changes (packages/ui/src/components/base/NavTabs.vue:254-267, 367-381) ‖ motion.md row 33 — element "controls — NavTabs indicator": `packages/ui/src/components/base/NavTabs.vue:367-381` left, right, top and bottom 80ms cubic-bezier(0.4, 0, 0.2, 1) with per-side v-bind delays; `.tab-color` 100ms plus a 100ms delay when delayed; `:283-293` transitions switch on only after the first layout | trigger: route change ‖ motion.md row 33: 80ms slide, colour 100ms starting 100ms later | crates/sift-ui/src/shell.rs:1245-1252 switches on press with no tween ‖ motion.md row 33: none in `crates/sift-ui/src` (library and skins view toggles are static) | no animation ‖ motion.md row 33: absent | OPEN | P1 |
| 66 | tabs — six kinds and order | `apps/app-frontend/src/pages/Browse.vue:713-735` | Modpacks, Mods, Resource Packs, Data Packs, Shaders, Servers (apps/app-frontend/src/pages/Browse.vue:713-735; strings 593-607) | static | crates/sift-ui/src/shell.rs:1213-1220 | none | DONE | — |
| 67 | tabs — Servers tab target | `apps/app-frontend/src/pages/Browse.vue:618, 733, 1078-1117` | projectType becomes 'server' and the server-search endpoint answers (apps/app-frontend/src/pages/Browse.vue:618, 733, 1078-1117) | press | crates/sift-ui/src/shell.rs:1219 maps Servers to the `plugin` project type | the Servers tab searches plugins | OPEN | P1 |
| 68 | search — field box | `packages/ui/src/components/base/inputs/InputFrame.vue:43-51` | `size="large"` → `h-12 rounded-[14px] px-4` (packages/ui/src/components/base/inputs/InputFrame.vue:43-51) | static | crates/sift-ui/src/shell.rs:2201-2204 (`h 48`) and crates/sift-ui/src/widgets/search.rs:201-207 (radius 14) | none: `size="large"` is `h-12 rounded-[14px] px-4`, and the field is drawn at 48 tall with a 14-point radius over the full 1028-point column, content `[88 126] - [1115 174]` against the reference's 88..1113 × 126..174. RE-VERIFIED BY DRIVING 2026-11-02 (`.scratch/cap/disc-search.png`: the plate is #34363c inside a #42444a ring) | DONE | P0 |
| 69 | search — field place in the page | `packages/ui/src/layouts/shared/browse-tab/layout.vue:147-161, 178` | its own full-width row (`wrapper-class="w-full"`), 8 below the strip and 8 above the sort row (packages/ui/src/layouts/shared/browse-tab/layout.vue:147-161, 178) | static | crates/sift-ui/src/shell.rs:2201-2208 (field on its own row at `top + 54`, the selects on the row below at `top + 110`) | none: the field spans the column on its own row with the two selects and the pager on the next one. RE-VERIFIED BY DRIVING 2026-11-02 | DONE | P0 |
| 70 | search — field surface | `packages/ui/src/components/base/inputs/InputFrame.vue:53-58` | `border-surface-5 bg-surface-4` plus an inset 1px surface-5 shadow (packages/ui/src/components/base/inputs/InputFrame.vue:53-58) | focus-within: 4 px ring `--color-brand-shadow` and text-contrast (InputFrame.vue:4) | crates/sift-ui/src/widgets/search.rs:201-207 (fill and ring), :251-262 (the caret) | the fill and the inset ring now match (`--surface-4` #34363c inside a 1 px `--surface-5` #42444a at radius 14), but the focus-within state is a caret and not the sheet's 4 px `--color-brand-shadow` ring (`InputFrame.vue:4`): `widgets/search.rs:197-200` says so in as many words. The ring is a `widgets/search.rs` change, not a page's. RE-VERIFIED 2026-11-02 | OPEN-WITH-CAUSE | P0 |
| 71 | search — leading glyph | `packages/ui/src/components/base/inputs/Input.vue:10-19` | 20-point search glyph in a `size-5` span, `text-secondary opacity-60` → `text-contrast opacity-100` on focus (packages/ui/src/components/base/inputs/Input.vue:10-19) | focus | crates/sift-ui/src/widgets/search.rs:185-193 a 12-point glyph centred 18 in | 12 vs 20; no opacity or focus change | OPEN | P1 |
| 72 | search — text | `packages/ui/src/components/base/inputs/Input.vue:51-55` | `text-base` (16) `font-medium text-primary`, placeholder `text-secondary`, focus `text-contrast` (packages/ui/src/components/base/inputs/Input.vue:51-55) | focus / typing | crates/sift-ui/src/widgets/search.rs:213-223 13-point; value inked `BASE`, placeholder `SECONDARY` | 13 vs 16; the value should be text-primary | OPEN | P0 |
| 73 | search — text inset | `packages/ui/src/components/base/inputs/InputFrame.vue:49`<br>`Input.vue:10, 54` | px-4 16 + the 20 glyph + gap-2 8 → text at 44 (packages/ui/src/components/base/inputs/InputFrame.vue:49; Input.vue:10, 54) | static | crates/sift-ui/src/widgets/search.rs:197 `text_left = 34` | 10 points too far left | OPEN | P1 |
| 74 | search — clear button | `packages/ui/src/components/base/inputs/Input.vue:63-69, 134-136`<br>`Input.vue:116` | 20-point `InputClearButton`, rendered only when `clearable && value` (packages/ui/src/components/base/inputs/Input.vue:63-69, 134-136), label "Clear input" (Input.vue:116) | press clears the text and emits `clear` → query "" and page 1 (layout.vue:160; use-browse-search.ts:414-417) | crates/sift-ui/src/widgets/search.rs:154-175 a 16-point X at right−18, drawn only while hovered | 16 vs 20; hover-driven instead of value-driven; no accessible label RE-VERIFIED 2026-11-01: the clear button is value-driven rather than hover-driven — it is drawn and interactive whenever the field holds text (widgets/search.rs:176-193) — and is still 16 points against the reference's 20 and still carries no accessible label. | OPEN-WITH-CAUSE | P1 |
| 75 | search — placeholder copy | `packages/ui/src/layouts/shared/browse-tab/layout.vue:50-53, 152-156` | "Search {projectType}..." → "Search modpacks...", "Search resource packs..." (packages/ui/src/layouts/shared/browse-tab/layout.vue:50-53, 152-156) | per tab | crates/sift-ui/src/widgets/search.rs:44 one constant "Search modpacks..." | fixed string that does not follow the tab | OPEN | P1 |
| 76 | search — request cadence | `packages/ui/src/layouts/shared/browse-tab/composables/use-browse-search.ts:301-314, 285-299, 408-421` | 200 ms debounce after any request parameter changes; page resets to 1 on query, sort, page-size or filter change; smooth scroll to top on paging and on filter change (packages/ui/src/layouts/shared/browse-tab/composables/use-browse-search.ts:301-314, 285-299, 408-421) | typing / press | crates/sift-ui/src/shell.rs:1312-1334 asks immediately on change, one request per keystroke; no scroll restore | no debounce; no scroll-to-top | OPEN | P2 |
| 77 | search — Enter key | `packages/ui/src/layouts/shared/browse-tab/layout.vue:147-161` | plain text input, no submit (packages/ui/src/layouts/shared/browse-tab/layout.vue:147-161) | key | crates/sift-ui/src/widgets/search.rs:123-133 Enter drops focus | invented unfocus behaviour | OPEN | P2 |
| 78 | sort — row composition | `packages/ui/src/layouts/shared/browse-tab/layout.vue:178-234` | one `flex flex-wrap items-center gap-2` row: sort 256 wide (`!w-[16rem]`), view 144 (`!w-[9rem]`), pager pushed right with `ml-auto` (packages/ui/src/layouts/shared/browse-tab/layout.vue:178-234) | static | crates/sift-ui/src/shell.rs:1354-1359 two 150-wide selects inside the 40-tall search row | wrong row and wrong widths (150/150 vs 256/144) | OPEN | P0 |
| 79 | sort — trigger box | `packages/ui/src/components/base/buttons/ButtonFrame.vue:31`<br>`Combobox.vue:341-344` | Combobox trigger defaults `md`: `h-9 rounded-xl px-2.5 gap-1.5 text-base font-semibold`, 20-point svg (packages/ui/src/components/base/buttons/ButtonFrame.vue:31; Combobox.vue:341-344) | hover brightness `--hover-brightness`; `active:scale-[0.97]`; 150 ms ease-out (ButtonFrame.vue:18-21) | crates/sift-ui/src/shell.rs:1370-1377 a surface-3 plate, radius 8, 13-point text inked `BASE` | 40 vs 36 tall; radius 8 vs 12; surface-3 vs surface-4 plus inset ring; 13 vs 16 text; no hover or press | OPEN | P0 |
| 80 | sort — prefix and value | `packages/ui/src/layouts/shared/browse-tab/layout.vue:190-194`<br>`Combobox.vue:78-83`<br>`packages/ui/src/utils/common-messages.ts:351-354` | two spans: "Sort by: " `font-semibold text-primary`, then the value `font-semibold leading-tight text-inherit` (packages/ui/src/layouts/shared/browse-tab/layout.vue:190-194; Combobox.vue:78-83; packages/ui/src/utils/common-messages.ts:351-354) | static | crates/sift-ui/src/shell.rs:1360-1369 one 13-point string "Sort by: {name}" | one run instead of two 16-point semibold runs | OPEN | P1 |
| 81 | sort — options | `packages/ui/src/utils/search.ts:389-395`<br>`use-browse-search.ts:427-429` | Relevance, Downloads, Followers, Date published, Date updated (packages/ui/src/utils/search.ts:389-395); default Relevance (397); reset to Relevance when the tab changes (use-browse-search.ts:427-429) | select | crates/sift-ui/src/shell.rs:1345-1352 "Follows", "Newest", "Updated" | three of five labels wrong . DRIVEN 2026-11-02: the select opens a five-row popover with the chosen row washed `bg-button-bg-selected`; the labels come from `SORTS` at `shell.rs:2285-2291`, where `Follows`, `Newest` and `Updated` stand for `Followers`, `Date published` and `Date updated` — a shell.rs change, recorded not done | OPEN | P1 |
| 82 | view — options | `packages/ui/src/layouts/shared/browse-tab/composables/use-browse-search.ts:144-146`<br>`packages/ui/src/utils/search.ts:387` | 5, 10, 15, 20, 50, 100; default 20 (packages/ui/src/layouts/shared/browse-tab/composables/use-browse-search.ts:144-146; packages/ui/src/utils/search.ts:387) | select | crates/sift-ui/src/shell.rs:1352 `SIZES` = 10, 20, 50, 100 | 5 and 15 missing . DRIVEN 2026-11-02: the view popover drew four rows (screen y 264, 296, 328, 360) where the reference has six; `SIZES` is `shell.rs:2292` | OPEN | P1 |
| 83 | view — label | `packages/ui/src/layouts/shared/browse-tab/layout.vue:54-57, 209-211` | "View: " prefix (packages/ui/src/layouts/shared/browse-tab/layout.vue:54-57, 209-211) | static | crates/sift-ui/src/shell.rs:1368 "View: {n}" | none on the string (both read "View: 20"); typography gap is row 25 | DONE | — |
| 84 | dropdown — panel | `packages/ui/src/components/base/Combobox.vue:98-113, 249-251, 1136-1151` | `rounded-[14px] bg-surface-4 border border-surface-5 shadow-2xl`, width = trigger width, 8 below the trigger, max-height 300, 8-point scrollbar (packages/ui/src/components/base/Combobox.vue:98-113, 249-251, 1136-1151) | open/close: opacity + scale over 125 ms cubic-bezier(0.16,1,0.3,1) (packages/ui/src/styles/tailwind-utilities.css:56-74) | crates/sift-ui/src/shell.rs:1585-1642 a `SUPER_RAISED_BG` plate, radius 8, 4 below, 32-tall rows | radius 8 vs 14; 4 vs 8 offset; no open animation; no max height | OPEN | P1 |
| 85 | dropdown — option row | `packages/ui/src/components/base/Combobox.vue:140, 491-502` | `px-4 py-3` (44 tall), gap-2.5, `transition-all 150 ms`; selected `bg-highlight-green text-green`; others `hover:brightness-[115%]` (packages/ui/src/components/base/Combobox.vue:140, 491-502) | hover / selected | crates/sift-ui/src/shell.rs:1606-1622 rows 32 tall, 10 inset, selected `BUTTON_BG_SELECTED` | 32 vs 44 tall; selected fill uses the wrong token | OPEN | P1 |
| 86 | dropdown — keyboard | `packages/ui/src/components/base/Combobox.vue:888-971, 1053-1056` | Enter/Space/ArrowDown open; Escape closes; arrows and Tab move; outside click closes (packages/ui/src/components/base/Combobox.vue:888-971, 1053-1056) | key | crates/sift-ui/src/shell.rs:1387-1397 press only | no keyboard operation | OPEN | P2 |
| 87 | pager — placement | `packages/ui/src/layouts/shared/browse-tab/layout.vue:228-233, 416-423` | Pagination `ml-auto` in the sort row and a second copy `flex justify-end mt-3` under the list (packages/ui/src/layouts/shared/browse-tab/layout.vue:228-233, 416-423) | static | crates/sift-ui/src/shell.rs:1413-1505 one pager inside the search row, nothing under the list | one of the two pagers is missing and it sits in the wrong row | OPEN | P0 |
| 88 | pager — cell | `packages/ui/src/components/base/Pagination.vue:2, 74-84` | quiet Button `!min-w-9 !rounded-full` → at least 36×36, cells `gap-1` (packages/ui/src/components/base/Pagination.vue:2, 74-84) | current page `!bg-brand-highlight` and brand ink with `aria-current=page`; its hover fills brand (Pagination.vue:66-70; ButtonFrame.vue:58-61) | crates/sift-ui/src/shell.rs:1427-1459 a 20×22 cell, radius 6, 12-point numeral | 20/22 vs 36/36; radius 6 vs 18; 12 vs 16 text; no hover . DRIVEN 2026-11-02: the current cell is a 36-point disc (screen x 954..990 at y 222) in `bg-button-bg-selected` with a 16-point numeral — the box, radius and type now match; the hover fill is still absent | OPEN | P0 |
| 89 | pager — page window | `packages/ui/src/components/base/Pagination.vue:140-171` | 1 … current−1, current, current+1 … last (packages/ui/src/components/base/Pagination.vue:140-171) | static | crates/sift-ui/src/shell.rs:1948-1973 the same window | none | DONE | — |
| 90 | pager — ellipsis | `packages/ui/src/components/base/Pagination.vue:33-60, 184-196` | a rotated ellipsis icon that swaps into a `w-14` number Input clamped to 1..count; Enter jumps, Escape cancels (packages/ui/src/components/base/Pagination.vue:33-60, 184-196) | press → input | crates/sift-ui/src/shell.rs:1432-1437 a static "…" | no jump-to-page control | OPEN | P2 |
| 91 | pager — arrows | `packages/ui/src/components/base/Pagination.vue:3-23, 88-108, 198-211`<br>`buttons/IconButton.vue:26-30` | ChevronLeft/ChevronRight IconButtons (md, icon-only, circular → 36×36) with tooltips "Previous page"/"Next page"; previous hidden on page 1 and next on the last page (packages/ui/src/components/base/Pagination.vue:3-23, 88-108, 198-211; buttons/IconButton.vue:26-30) | hover: quiet surface | crates/sift-ui/src/shell.rs:1478-1503 ASCII "<" and ">" in 22-point boxes, always drawn, only the ink changes | wrong glyphs; no tooltips; disabled instead of hidden . DRIVEN 2026-11-02: the arrows are still `<` and `>` at screen x≈940 and x≈1104 on the sort row, and a press on `>` did turn the page (rms 29.2 with the worst tiles in the card column), so they are wired — wrong glyphs, no tooltip, and hidden-vs-disabled is still row 91's gap (`shell.rs:2449-2474`) | OPEN | P1 |
| 92 | pager — visibility | `packages/ui/src/components/base/Pagination.vue:2` | the component renders nothing unless `count > 1` (packages/ui/src/components/base/Pagination.vue:2) | static | crates/sift-ui/src/shell.rs:1413-1425 prints "No results" where the strip would be | prints a label the reference does not have | OPEN | P1 |
| 93 | toolbar — result count | `packages/ui/src/layouts/shared/browse-tab/layout.vue:129-424`<br>`use-browse-search.ts:195-198` | no "1 - 20 of N" element anywhere in the layout; totalHits only feeds `pageCount` and the sidebar (packages/ui/src/layouts/shared/browse-tab/layout.vue:129-424; use-browse-search.ts:195-198) | static | crates/sift-ui/src/shell.rs:857-867 and 1413-1425 print a right-aligned "1 - 20 of 946" | invented element, visible at rest | OPEN | P1 |
| 94 | chips — row | `packages/ui/src/components/search/SearchFilterControl.vue:2` | `flex flex-wrap items-center gap-1 empty:hidden` between the sort row and the list (packages/ui/src/components/search/SearchFilterControl.vue:2) | static | none: the list starts right after the pager row (crates/sift-ui/src/shell.rs:1508-1511) | whole row missing | OPEN | P1 |
| 95 | chips — clear all | `packages/ui/src/components/search/SearchFilterControl.vue:3-10, 237-239` | TagItem "Clear all filters" with XCircleIcon, shown when more than one filter is selected; empties the filter list (packages/ui/src/components/search/SearchFilterControl.vue:3-10, 237-239) | press: `active:scale-[0.95]` and hover underline (packages/ui/src/components/base/TagItem.vue:4) | none | missing | OPEN | P1 |
| 96 | chips — selected filter pill | `packages/ui/src/components/base/TagItem.vue:18-19`<br>`SearchFilterControl.vue:28-36, 219-223` | TagItem `px-2 py-1 rounded-full border-1 surface-5 text-sm` with XIcon and the option name; negative filters prepend a red BanIcon; a press removes that one filter (packages/ui/src/components/base/TagItem.vue:18-19; SearchFilterControl.vue:28-36, 219-223) | hover underline | none | missing | OPEN | P1 |
| 97 | chips — locked pill | `packages/ui/src/components/search/SearchFilterControl.vue:37-51`<br>`apps/app-frontend/src/pages/Browse.vue:560-567` | provided filters render a LockIcon pill on the raised background with the "Filter locked" / "Game version is provided by the instance" tooltip (packages/ui/src/components/search/SearchFilterControl.vue:37-51; apps/app-frontend/src/pages/Browse.vue:560-567) | static | none | missing (no provided-filter concept) | OPEN | P2 |
| 98 | sidebar — filter groups | `packages/ui/src/utils/search.ts:456-763`<br>`sidebar.vue:244-303` | one accordion per filter type: category groups, Includes (modpack) / Depends on, Environment, Game version, Loader, License, Advanced exclusions last (packages/ui/src/utils/search.ts:456-763, ordering at 763; sidebar.vue:244-303) | open by default in the app variant: category*, environment, license (packages/ui/src/layouts/shared/browse-tab/sidebar.vue:121-123) | crates/sift-ui/src/panel.rs:500-570 a single "Category" heading, four flat names and See more | about seven collapsible groups replaced by one short list | OPEN | P0 |
| 99 | sidebar — group header | `packages/ui/src/layouts/shared/browse-tab/sidebar.vue:76-91, 233-237`<br>`SearchSidebarFilter.vue:11-19` | app variant: `px-3 py-3` button (first child `pt-4`), 1 px bottom rule in brand-gradient-border, `h3 text-base`, chevron `ml-auto size-5` (packages/ui/src/layouts/shared/browse-tab/sidebar.vue:76-91, 233-237; SearchSidebarFilter.vue:11-19) | chevron rotate-180 over 300 ms; title ink to contrast on group hover (SearchSidebarFilter.vue:16; Accordion.vue:13) | crates/sift-ui/src/panel.rs:500-506 a heading with no rule, chevron or press target | headers are not controls and cannot collapse | OPEN | P1 |
| 100 | sidebar — group body | `packages/ui/src/layouts/shared/browse-tab/sidebar.vue:93-94`<br>`SearchSidebarFilter.vue:117` | `mt-2 mb-3` with an `ml-2 mr-3` inner panel and `gap-1` rows (packages/ui/src/layouts/shared/browse-tab/sidebar.vue:93-94; SearchSidebarFilter.vue:117) | accordion opens 0fr→1fr grid rows over 0.3 s ease-in-out and is `inert` while closed (packages/ui/src/components/base/Accordion.vue:43-53, 125-139) | crates/sift-ui/src/panel.rs has no body or disclosure state | no open/close state and no animation | OPEN | P1 |
| 101 | sidebar — option row | `packages/ui/src/components/search/SearchFilterOption.vue:4` | `rounded-xl px-2 py-1 text-sm font-semibold`; idle `text-secondary`, hover `bg-button-bg text-contrast`; included `bg-brand-highlight text-contrast`; excluded `bg-highlight-red` (packages/ui/src/components/search/SearchFilterOption.vue:4) | check or ban icon 16 at the row's right fades in on hover (SearchFilterOption.vue:8-17, 106-112); `active:scale-[0.98]` | crates/sift-ui/src/panel.rs:512-544 plain 13-point rows, brand ink when chosen, hover changes ink only | plain text rows instead of chips with selected fills and action icons | OPEN | P1 |
| 102 | sidebar — option icons | `packages/ui/src/components/search/SearchSidebarFilter.vue:147-163, 357-371` | each option carries its icon at 16×16, loaders coloured from `--color-platform-*` (packages/ui/src/components/search/SearchSidebarFilter.vue:147-163, 357-371) | static | crates/sift-ui/src/panel.rs:512-544 text only | icons missing | OPEN | P2 |
| 103 | sidebar — game version group | `packages/ui/src/components/search/SearchSidebarFilter.vue:103-116, 353-355` | `scrollable` at 10+ options: `h-[16rem]` ScrollablePanel with a small search Input above it (packages/ui/src/components/search/SearchSidebarFilter.vue:103-116, 353-355) | typing filters the rows live (SearchSidebarFilter.vue:411-424) | none | group and its search absent | OPEN | P2 |
| 104 | sidebar — show more | `packages/ui/src/components/search/SearchSidebarFilter.vue:189-203` | `expandable` groups hide non-default options behind "Show more"/"Show fewer" with a rotating chevron (packages/ui/src/components/search/SearchSidebarFilter.vue:189-203) | press | crates/sift-ui/src/panel.rs:546-570 "See more"/"See less" grows the category list instead | wrong control attached to the wrong list | OPEN | P2 |
| 105 | card — container | `packages/ui/src/components/project/card/ProjectCard.vue:258` | `rounded-2xl border-1 border-surface-4 bg-surface-3 overflow-hidden` (packages/ui/src/components/project/card/ProjectCard.vue:258) | hover: whole card `filter: brightness(1.25)`; press: `scale(0.98)`; contents `transition: scale 125 ms ease-out` (SmartClickable.vue:48-87; variables.scss:407) | crates/sift-ui/src/shell.rs:1667-1669 radius 12, `RAISED_BG`, no hover or press | radius 12 vs 16; no brightness, scale or focus ring | OPEN | P0 |
| 106 | card — list layout | `packages/ui/src/components/project/card/ProjectCard.vue:100-107, 311-325` | `p-4` grid `icon info stats stats / icon info dummy stats / icon tags tags stats` with `gap-x-3 gap-y-2`, height content-driven (packages/ui/src/components/project/card/ProjectCard.vue:100-107, 311-325) | static | crates/sift-ui/src/shell.rs:1667 a fixed 148 tall card with parts at absolute 16/18/44/104 | a hand-placed 148-point card instead of a content-sized grid | OPEN | P0 |
| 107 | card — icon | `packages/ui/src/components/project/card/ProjectCard.vue:108-113`<br>`packages/ui/src/components/base/Avatar.vue:282-294`<br>`ProjectCard.vue:347-351` | Avatar `100px` in the list layout, radius 16/96 of its size, 1 px white/15% outline, bg button-bg (packages/ui/src/components/project/card/ProjectCard.vue:108-113; packages/ui/src/components/base/Avatar.vue:282-294); 64 only inside a container narrower than 850 (ProjectCard.vue:347-351) | hover: opacity .85 over 125 ms (SmartClickable.vue:71-74) | crates/sift-ui/src/shell.rs:1673-1686 a 64-point plate at radius 12 | 64 vs 100 at 1440 (64 is the narrow-container branch); radius 12 vs 16.7; no hover dim | OPEN | P0 |
| 108 | card — title | `packages/ui/src/components/project/card/ProjectCardTitle.vue:10-13` | `text-xl` (20) semibold `leading-none` text-contrast, one line, break-all (packages/ui/src/components/project/card/ProjectCardTitle.vue:10-13) | hover: underline (SmartClickable.vue:59-61) | crates/sift-ui/src/shell.rs:1689-1695 a 17-point title 18 down, no underline | 17 vs 20; no hover underline | OPEN | P0 |
| 109 | card — author | `packages/ui/src/components/project/card/ProjectCardAuthor.vue:19-38` | "by {name}", `text-secondary font-normal`, one line, external glyph only for http links (packages/ui/src/components/project/card/ProjectCardAuthor.vue:19-38) | hover: the name underlines and brightens (ProjectCardAuthor.vue:32) | crates/sift-ui/src/shell.rs:1696-1711 a 13-point "by …" with an always-drawn external glyph | 13 vs 16; glyph unconditional; no per-name hover | OPEN | P1 |
| 110 | card — summary | `packages/ui/src/components/project/card/ProjectCard.vue:120-122` | `line-clamp-2` at the card's inherited 16/24 across the info column (packages/ui/src/components/project/card/ProjectCard.vue:120-122) | static | crates/sift-ui/src/shell.rs:1716-1726 13-point, width `min(rect−246, 440)` and cut at 150 characters | 13 vs 16; a 440-point measure and a character cut the source does not state | OPEN | P1 |
| 111 | card — tag pills | `packages/ui/src/components/base/TagItem.vue:18-19`<br>`ProjectCard.vue:168-176`<br>`ProjectCardTags.vue:43-63` | TagTagItem `px-2 py-1 rounded-full border-1 surface-5 bg-button-bg text-sm leading-none gap-1` with a 16-point icon (packages/ui/src/components/base/TagItem.vue:18-19); 4 tags with actions, 5 when the environment row is absent (ProjectCard.vue:168-176; ProjectCardTags.vue:43-63) | `+N` pill opens a tooltip listing the hidden tags (packages/ui/src/components/project/TagsOverflow.vue:16-27) | crates/sift-ui/src/shell.rs:1731-1789 11-point labels in 24-tall pills, 6-point gaps, `+N` with no tooltip | 11 vs 14 text; 6 vs 4 gap; no tooltip; no loader icons | OPEN | P1 |
| 112 | card — stats | `packages/ui/src/components/project/card/ProjectCardStats.vue:18-49`<br>`ProjectCard.vue:92` | DownloadIcon and HeartIcon at `size-5` with a `font-medium` compact count, `gap-2` per stat and `gap-3` between them (packages/ui/src/components/project/card/ProjectCardStats.vue:18-49; ProjectCard.vue:92) | tooltip "N downloads" / "N followers" (ProjectCardStats.vue:18-27) | crates/sift-ui/src/shell.rs:1804-1838 11-point counts, 12-point glyphs, order downloads/heart/calendar | 11 vs 16 text; 12 vs 20 glyph; no tooltips | OPEN | P1 |
| 113 | card — date | `packages/ui/src/components/project/card/ProjectCardDate.vue:21-49`<br>`layout.vue:357-359` | History or Calendar icon `size-5` plus the capitalised relative time, tooltip "Updated {date}" / "Published {date}" (packages/ui/src/components/project/card/ProjectCardDate.vue:21-49); published when sorted by newest, otherwise updated (layout.vue:357-359) | static | crates/sift-ui/src/shell.rs:1554-1556 leaves `updated` empty for live hits, so no date is drawn | no date on any real card | OPEN | P1 |
| 114 | card — install action | `packages/ui/src/layouts/shared/browse-tab/layout.vue:80-99, 397-408`<br>`ButtonFrame.vue:31, 166-168` | Button `outlined` brand at `md` (h-9, rounded-xl, px-2.5, 16-point text, 20-point icon) with `!shadow-[inset_0_0_0_1px_var(--color-brand)]` (packages/ui/src/layouts/shared/browse-tab/layout.vue:80-99, 397-408; ButtonFrame.vue:31, 166-168) | Install → Installing with a spinning icon → Installed with a check, disabled while installed or installing; "Add to an instance" with no instance (apps/app-frontend/src/pages/Browse.vue:997-1013) | crates/sift-ui/src/shell.rs:1792-1803 a 96×36 filled `BUTTON_BG_SELECTED` plate; crates/sift-ui/src/install.rs:184-194 labels Install / Installing… / Installed | filled instead of outlined brand; fixed 96 width; an ellipsis the source does not use; no spinner or check glyph | OPEN | P0 |
| 115 | card — hover reveal | `packages/ui/src/components/project/card/ProjectCard.vue:125-130, 379-411`<br>`SmartClickable.vue:48-75` | nothing is hidden: the action slot is always in the grid (packages/ui/src/components/project/card/ProjectCard.vue:125-130, 379-411); hover only brightens, underlines the title and dims the icon (SmartClickable.vue:48-75) | hover 125 ms ease-out; press scale .98; focus-visible 4 px focus-ring outline (SmartClickable.vue:77-82) | crates/sift-ui/src/shell.rs:1658-1862 one static state | no hover or focus response at all | OPEN | P0 |
| 116 | card — banner strip | `packages/ui/src/layouts/shared/browse-tab/composables/use-browse-search.ts:156-161`<br>`ProjectCard.vue:11-28` | grid layout only; `displayMode` is unset, so the app always renders `list` (packages/ui/src/layouts/shared/browse-tab/composables/use-browse-search.ts:156-161; ProjectCard.vue:11-28) | static | none | not reachable in the app variant | NOT-IN-SOURCE | P2 |
| 117 | state — loading | `packages/ui/src/layouts/shared/browse-tab/layout.vue:259-261`<br>`packages/ui/src/components/base/LoadingIndicator.vue:1-113` | `LoadingIndicator`: "Loading" with a 2 s dot cycle over three 4 rem placeholder bars at `--radius-lg`, opacity .25, `pop` and `shimmer` 4 s with a .3 s stagger (packages/ui/src/layouts/shared/browse-tab/layout.vue:259-261; packages/ui/src/components/base/LoadingIndicator.vue:1-113) | loop 4 s | crates/sift-ui/src/shell.rs:1528-1563 while `Loading` the list is empty, so the sample cards are drawn | six fabricated project cards appear where the skeleton belongs | OPEN | P0 |
| 118 | state — no results | `packages/ui/src/layouts/shared/browse-tab/layout.vue:265-274, 66-69` | centred "No results found for your query!" in `section.offline text-center` (packages/ui/src/layouts/shared/browse-tab/layout.vue:265-274, 66-69) | static | crates/sift-ui/src/shell.rs:1562-1563 falls back to the sample cards; 1418-1425 prints "No results" in the pager slot | message missing from the list, stray label in the toolbar | OPEN | P0 |
| 119 | state — offline | `packages/ui/src/layouts/shared/browse-tab/layout.vue:262-264, 62-65`<br>`apps/app-frontend/src/pages/Browse.vue:530-545` | "You are currently offline. Connect to the internet to browse Modrinth!" when offline and totalHits is 0 (packages/ui/src/layouts/shared/browse-tab/layout.vue:262-264, 62-65; apps/app-frontend/src/pages/Browse.vue:530-545) | static | no offline detection in crates/sift-ui/src/shell.rs | missing | OPEN | P2 |
| 120 | state — error | `packages/ui/src/layouts/shared/browse-tab/composables/use-browse-search.ts:371-378` | no dedicated error UI: a failed request leaves the previous hits on screen and clears loading/refreshing, logging to the console (packages/ui/src/layouts/shared/browse-tab/composables/use-browse-search.ts:371-378); UNKNOWN whether a toast is also raised — a capture during a failed request would settle it | static | crates/sift-ui/src/shell.rs:1528-1560 `State::Failed` lands in the sample fallback | fabricated cards on failure | OPEN | P0 |
| 121 | state — empty query | `apps/app-frontend/src/pages/Browse.vue:1201`<br>`use-browse-search.ts:323-379` | the default feed is requested on mount with no text, via `refreshSearch` (apps/app-frontend/src/pages/Browse.vue:1201; use-browse-search.ts:323-379) | mount | crates/sift-ui/src/search.rs:118-124 refuses to ask with no text and no facet, so nothing is requested until a key is pressed | the reference opens on a full catalogue; ours opens on fabricated cards | OPEN | P0 |
| 122 | unknown — `.search` block and `.offline` styling | `packages/ui/src/layouts/shared/browse-tab/layout.vue:258-274` | the classes are used (packages/ui/src/layouts/shared/browse-tab/layout.vue:258-274) but no rule for them exists in the vendored tree — not stated in source | — | crates/sift-ui/src/shell.rs:1508-1511 uses a 12-point gap only | UNKNOWN: a 1440×900 capture of /browse/modpack, or the app's compiled stylesheet, would settle the block's margins and the empty-state placement | OPEN | P2 |
| 123 | unknown — capture of this page | — (the report gives no file:line) | no Discover capture exists in .scratch/cap — only ref-home.png and ours-home*.png, which are Home | — | `.scratch/cap/ref-discover.png` (1440×900, window at 0,0) and `.scratch/cap/disc-search.png` for ours | none: the capture this row called missing exists. RE-VERIFIED 2026-11-02 — tab strip y 73..116, search field y 126..172, sort row y 182..218, first card y 231, and the reference's own pager on that row | DONE | P2 |
| 124 | legacy — LegacyProjectCard | `apps/app-frontend/src/components/ui/LegacyProjectCard.vue` (no line number in the report) | apps/app-frontend/src/components/ui/LegacyProjectCard.vue exists but nothing in apps or packages imports it (no reference found) — not stated in source which route used it | — | not implemented | UNKNOWN why the brief lists it; the shipped Discover card is packages/ui/src/components/project/card/ProjectCard.vue (layout.vue:340-412) | NOT-IN-SOURCE | P2 |

## 4. Instance

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 125 | header — header block padding | `apps/app-frontend/src/pages/instance/layout.vue:3-6` | apps/app-frontend/src/pages/instance/layout.vue:3-6 (p-6 pr-2 pb-4 = 24px top/left, 8px right, 16px bottom) | static | crates/sift-ui/src/pages/instance.rs:37,218-220 (HEADER_HEIGHT 84; text at rect.top()+NAME_TOP, NAME_TOP=4 at :70, flagged guess) | no padding model: name starts 4px down (24 in ref); right edge flush 0px (8 in ref) | OPEN | P0 |
| 126 | header — divider under header | `packages/ui/src/components/base/page-header/index.vue:2,59-63` | packages/ui/src/components/base/page-header/index.vue:2,59-63 (border-b 1px border-divider + pb-4) | always on (divider defaults true, :52) | crates/sift-ui/src/pages/instance.rs:203-320 | no divider; no 16px gap before the tab strip | OPEN | P1 |
| 127 | header — instance icon | `packages/ui/src/components/base/page-header/index.vue:4-10` | packages/ui/src/components/base/page-header/index.vue:4-10 (Avatar size 64px, pad-transparent-corners, tint-by instance.id) | static | crates/sift-ui/src/pages/instance.rs:211-213 (shortcut: no icon tile) | missing | OPEN | P0 |
| 128 | header — instance name | `packages/ui/src/components/base/page-header/index.vue:13` | packages/ui/src/components/base/page-header/index.vue:13 (h1 text-2xl = 24px, font-semibold, leading-none, text-contrast) | truncates only with truncateTitle (:64) | crates/sift-ui/src/pages/instance.rs:66,218-224 (24px semibold) | none | DONE | P2 |
| 129 | header — loader / game version metadata | `packages/ui/src/components/base/page-header/index.vue:44-51,320-322`<br>`packages/ui/src/components/base/page-header/metadata/page-header-metadata-item.vue:79-82` | packages/ui/src/components/base/page-header/index.vue:44-51,320-322 (TagIcon loader tag + '{loader} {game_version}') ; packages/ui/src/components/base/page-header/metadata/page-header-metadata-item.vue:79-82 (no text-* class, so 16px root) | tooltip 'Mod loader and Minecraft version' (:48); hover:underline when linked (:82) | crates/sift-ui/src/pages/instance.rs:225-231,674-683 (13px plain line) | 13px vs 16px implied; no loader tag icon; no tooltip | OPEN | P1 |
| 130 | header — playtime | `packages/ui/src/components/base/page-header/index.vue:52-58,330-347`<br>`apps/app-frontend/src/composables/use-app-settings.ts:47` | packages/ui/src/components/base/page-header/index.vue:52-58,330-347 ('N hours'/'N minutes'/'N seconds'), gated by showPlayTime (apps/app-frontend/src/composables/use-app-settings.ts:47) | hidden when 0 seconds (:332-334) | crates/sift-ui/src/pages/instance.rs:684-701 (no playtime row) | missing | OPEN | P1 |
| 131 | header — last played | `packages/ui/src/components/base/page-header/index.vue:59-67` | packages/ui/src/components/base/page-header/index.vue:59-67 (relative time item, else 'Never played') | relative label, tooltip on hover | crates/sift-ui/src/pages/instance.rs:684-703 ('Last played' row, value is an em dash) | no relative time, no clock icon, em dash invented | OPEN | P0 |
| 132 | header — metadata separators | `packages/ui/src/components/base/page-header/metadata/index.vue:4`<br>`metadata/page-header-metadata-item.vue:4-7` | packages/ui/src/components/base/page-header/metadata/index.vue:4 (gap-x-[1.625rem] = 26px, gap-y-2) ; metadata/page-header-metadata-item.vue:4-7 (BulletDivider in a 26px gutter) | first item of a wrapped row drops its bullet | none | missing dots and 26px gutters | OPEN | P1 |
| 133 | header — Play button | `packages/ui/src/components/base/page-header/index.vue:132-142`<br>`packages/ui/src/components/base/buttons/ButtonFrame.vue:33` | packages/ui/src/components/base/page-header/index.vue:132-142 ; packages/ui/src/components/base/buttons/ButtonFrame.vue:33 (xl: h-12, rounded-2xl = 16px, px-3.5, gap-2, text-base font-extrabold, svg size-6), :46-47 (colored: bg brand + accent-contrast text), :142-164 (shadow + 1px gradient ring) | hover brightness (:21), active scale .97 (:19), 150ms ease-out (:18), disabled opacity 50 (:24) | crates/sift-ui/src/pages/instance.rs:98-100,243-263 (96x40, RADIUS_SM = 8 via crates/sift-ui/src/theme.rs:791, BUTTON_BG_SELECTED + BUTTON_TEXT_SELECTED, 16px icon, 14px label) | height 40 vs 48, radius 8 vs 16, translucent green vs solid brand, green label vs accent-contrast, icon 16 vs 24, no states | OPEN | P0 |
| 134 | header — Play button states | `packages/ui/src/components/base/page-header/index.vue:73-145` | packages/ui/src/components/base/page-header/index.vue:73-145 (Stop red + Stopping; Installing...; locked Play; Repair; server SplitButton; Starting...) | six exclusive states driven by playing/loading/stopping (layout.vue:306,360-363,684-700) | crates/sift-ui/src/pages/instance.rs:243-263 (one static label); crates/sift-ui/src/install.rs:29-61 tracks 6 Status variants, unread here | none of the six states exist | OPEN | P0 |
| 135 | header — settings IconButton | `packages/ui/src/components/base/page-header/index.vue:147-155`<br>`buttons/IconButton.vue:26-29`<br>`buttons/ButtonFrame.vue:36-42` | packages/ui/src/components/base/page-header/index.vue:147-155 ; buttons/IconButton.vue:26-29 (type base, circular) ; buttons/ButtonFrame.vue:36-42 (xl w-12 = 48px) | tooltip 'Instance settings' (:148); hover bg-surface-4 (ButtonFrame.vue:57); 150ms (:18) | absent | missing control | OPEN | P0 |
| 136 | header — overflow menu | `packages/ui/src/components/base/page-header/index.vue:156-164`<br>`buttons/button-menu/button-menu.ts:14-18` | packages/ui/src/components/base/page-header/index.vue:156-164 (quiet, xl) ; buttons/button-menu/button-menu.ts:14-18 (items min-h-10, px-3 py-2, rounded-[10px], text-base semibold), :20-23 (panel rounded-[14px], bg-surface-3, ring surface-5, p-2) | items Open folder / Export modpack / Create shortcut (+ Report) page-header/index.vue:356-397; dividers my-1 h-px TeleportOverflowMenu.vue:198; open/close duration not stated in source | absent | missing menu and its actions | OPEN | P0 |
| 137 | header — right-click context menu | `apps/app-frontend/src/pages/instance/layout.vue:816-865` | apps/app-frontend/src/pages/instance/layout.vue:816-865 (Stop / Play / Add content / divider / Edit / Open folder / Copy path) | items shown per playing and quarantined (:824,832,840) | absent | missing | OPEN | P1 |
| 138 | header — status badges | `packages/ui/src/components/base/page-header/index.vue:13-31`<br>`page-header/page-header-badge-item.vue:72` | packages/ui/src/components/base/page-header/index.vue:13-31 (Locked orange / Shared blue) ; page-header/page-header-badge-item.vue:72 (rounded-full, border surface-5, bg-button-bg, px-2 py-1, text-sm semibold) | linked badge hover:underline (:73) | absent | not drawn | OPEN | P2 |
| 139 | tabs — strip container | `packages/ui/src/components/base/NavTabs.vue:9-11` | packages/ui/src/components/base/NavTabs.vue:9-11 (nav flex w-fit rounded-full bg-bg-raised p-1 text-xs sm:text-sm font-bold + card-shadow border surface-4) | static | crates/sift-ui/src/pages/instance.rs:266-303 (bare pills on the page surface) | no container, radius, bg, border or 4px inset | OPEN | P0 |
| 140 | tabs — tab item | `packages/ui/src/components/base/NavTabs.vue:24,32,35` | packages/ui/src/components/base/NavTabs.vue:24,32,35 (px-4 py-2 gap-2; icon size-5 hidden below sm; label text-nowrap; pill approx 36px, derived) | colours NavTabs.vue:163-175 (active icon+label text-button-textSelected; inactive icon text-secondary, label text-contrast) | crates/sift-ui/src/pages/instance.rs:271-292 (width = label + 28, height 30, no icon, inactive label theme.BASE) | 14px vs 16px side padding, 30 vs ~36px height, no 8px icon gap or icons, inactive label grey vs contrast | OPEN | P0 |
| 141 | tabs — active pill | `packages/ui/src/components/base/NavTabs.vue:63-72` | packages/ui/src/components/base/NavTabs.vue:63-72 (absolute slider rounded-full bg-button-bgSelected sized to the active link) | left/right/top/bottom 80ms cubic-bezier(0.4,0,0.2,1) with 175ms stagger (NavTabs.vue:260-267,367-373); colour 100ms delayed 100ms (:374-381) | crates/sift-ui/src/pages/instance.rs:277-280 (paints the active pill in place) | no slider, no slide or colour animation | OPEN | P1 |
| 142 | tabs — tab press feedback | `packages/assets/styles/defaults.scss:97-107` | packages/assets/styles/defaults.scss:97-107 (active scale .95; opacity .5s, filter .2s, transform .05s, outline .2s) | on mouse press | none | no press feedback | OPEN | P2 |
| 143 | tabs — tab set and order | `apps/app-frontend/src/pages/instance/layout.vue:513-561`<br>`apps/app-frontend/src/composables/use-app-settings.ts:54-56`<br>`af/pages/instance/screenshots/index.vue:10`<br>`af/composables/use-app-settings.ts:56`<br>`af/pages/instance/layout.vue:530-536` | apps/app-frontend/src/pages/instance/layout.vue:513-561 (Content, Files, Screenshots, Worlds, Logs, Share), gates at :522,530,538,552; defaults files+worlds on, screenshots off (apps/app-frontend/src/composables/use-app-settings.ts:54-56) ‖ file-management.md row 45 — element "Screenshots — Per-instance Screenshots tab": Same component with `:instance-id` (`af/pages/instance/screenshots/index.vue:10`), tab gated by `showScreenshotsTabInInstances`, default **false** (`af/composables/use-app-settings.ts:56`; `af/pages/instance/layout.vue:530-536`). | Share only when shared instances are available (layout.vue:501-511) ‖ file-management.md row 45: — (this report has no states column) | crates/sift-ui/src/pages/instance.rs:155-161,169-175 (all five, Screenshots third) ‖ file-management.md row 45: Its own tab drawing up to 9 thumbnails, always shown (`sift-ui/pages/instance.rs:373-420`; shortcut noted at `:155-157`). | Screenshots shown though default-off; order puts Screenshots before Worlds ‖ file-management.md row 45: No per-instance file actions; visibility differs from the reference default. | OPEN | P0 |
| 144 | tabs — tab body | `apps/app-frontend/src/pages/instance/layout.vue:57-58,72`<br>`layout.vue:486-499`<br>`apps/app-frontend/src/routes.js:157-159` | apps/app-frontend/src/pages/instance/layout.vue:57-58,72 (px-6 strip; body p-6 pt-4 = 24 sides, 16 top); fixed render only for Logs (layout.vue:486-499; apps/app-frontend/src/routes.js:157-159) | Logs body scrolls in its own container, other tabs scroll the page | crates/sift-ui/src/pages/instance.rs:56,306-310 (card at strip bottom + 14px, full height, never scrolls) | gap 14 vs 16, no scroll region, no fixed mode for Logs | OPEN | P1 |
| 145 | content — layout root | `packages/ui/src/layouts/shared/content-tab/layout.vue:1088-1089` | packages/ui/src/layouts/shared/content-tab/layout.vue:1088-1089 (flex flex-col gap-4 + pb-6) | static | crates/sift-ui/src/pages/instance.rs:323-369 (invented two-column 'Description' + info rows; heading at :341) | structure does not exist in the reference RE-VERIFIED 2026-11-01: the invented two-column Description panel is gone from the tab — `info_rows` survives only in its own tests (instance.rs:1011, :1099-1102) — and what the tab draws now is a server-join field (:515-569) and one row per installed mod (:571-615); the reference's managed-content card and Additional content heading are still absent, which is what this row is for. | OPEN-WITH-CAUSE | P0 |
| 146 | content — managed content card | `packages/ui/src/layouts/shared/content-tab/layout.vue:18-29` | packages/ui/src/layouts/shared/content-tab/layout.vue:18-29 (pack or shared manager, with view/settings/primary action) | installing and disabled states (:21-22) | absent | missing | OPEN | P1 |
| 147 | content — 'Additional content' heading | `packages/ui/src/layouts/shared/content-tab/layout.vue:33-35` | packages/ui/src/layouts/shared/content-tab/layout.vue:33-35 (text-xl font-semibold text-contrast, mb-2) | only drawn beside a managed card | absent | missing | OPEN | P2 |
| 148 | content — search input | `packages/ui/src/layouts/shared/content-tab/layout.vue:38-57`<br>`packages/ui/src/components/base/inputs/InputFrame.vue:48` | packages/ui/src/layouts/shared/content-tab/layout.vue:38-57 ; packages/ui/src/components/base/inputs/InputFrame.vue:48 (medium h-10, rounded-[14px], px-4) | clearable; focus ring 4px brand-shadow (InputFrame.vue:4) | absent | missing | OPEN | P0 |
| 149 | content — Upload files / Browse content | `packages/ui/src/layouts/shared/content-tab/layout.vue:60-86`<br>`buttons/ButtonFrame.vue:32` | packages/ui/src/layouts/shared/content-tab/layout.vue:60-86 ; buttons/ButtonFrame.vue:32 (lg h-10, rounded-[14px]) | disabled with tooltip when busy or quarantined (:67,81) | absent | missing | OPEN | P0 |
| 150 | content — sort menu | `packages/ui/src/layouts/shared/content-tab/layout.vue:93-106` | packages/ui/src/layouts/shared/content-tab/layout.vue:93-106 (!h-[34px] !text-sm, sort icon + DropdownIcon, divider h-6 w-px) | four sort modes (:103-118) | absent | missing | OPEN | P1 |
| 151 | content — type filter pills | `packages/ui/src/layouts/shared/content-tab/layout.vue:107-115` | packages/ui/src/layouts/shared/content-tab/layout.vue:107-115 (FilterPills, All + per-type chips) | — | absent | missing | OPEN | P1 |
| 152 | content — content table header | `packages/ui/src/layouts/shared/content-tab/components/ContentCardTable.vue:186-195,226-269` | packages/ui/src/layouts/shared/content-tab/components/ContentCardTable.vue:186-195,226-269 (sticky h-12 px-3 bg-surface-3 rounded-t-[20px]; 16px chevrons) | stuck drops radius and border-t0 (:187,195) | absent | missing | OPEN | P0 |
| 153 | content — content table row | `packages/ui/src/layouts/shared/content-tab/components/ContentCardTable.vue:95,315-325`<br>`ContentCardItem.vue:178` | packages/ui/src/layouts/shared/content-tab/components/ContentCardTable.vue:95,315-325 (74px rows; alt bg-surface-2 / surface-1.5; selected surface-2.5; border-t surface-4) ; ContentCardItem.vue:178 (px-3 gap-4) | hover, installing, update, selected states (:316-320) | absent | missing RE-VERIFIED 2026-11-01: the Content tab draws one row per installed mod with its name and size, and a press renames the file to `.jar.disabled` (instance.rs:571-615): a row exists, the reference's table columns, checkbox and row menu do not. | OPEN-WITH-CAUSE | P0 |
| 154 | content — empty state | `packages/ui/src/layouts/shared/content-tab/layout.vue:1416-1469` | packages/ui/src/layouts/shared/content-tab/layout.vue:1416-1469 (EmptyState empty-inbox, 'No content installed', Upload + Browse) | filtered-empty variant 'No content found.' (:1409-1411) | crates/sift-ui/src/pages/instance.rs:338-368 (Description + rows) | wrong content for this tab RE-VERIFIED 2026-11-01: the tab's empty state is its own sentence rather than the Description panel — "No mods in this instance yet. Install one from Discover." (instance.rs:508-511); the reference's illustration and copy are still to port. | OPEN-WITH-CAUSE | P0 |
| 155 | content — selection bar | `packages/ui/src/layouts/shared/content-tab/components/ContentSelectionBar.vue` (no line number in the report) | packages/ui/src/layouts/shared/content-tab/components/ContentSelectionBar.vue (count, clear, bulk actions) | shown while items are selected | absent | missing | OPEN | P2 |
| 156 | files — navbar | `packages/ui/src/layouts/shared/files-tab/components/FileNavbar.vue:19-94` | packages/ui/src/layouts/shared/files-tab/components/FileNavbar.vue:19-94 (Home IconButton + text-sm breadcrumb, 16px chevrons), :97-109 (search Input 280px), :111-125 (Refresh outlined lg), :127-192 ('+' dropdown outlined lg) | editing appends the file name (:86-90); breadcrumb auto-scrolls when overflowing (:42-56) | crates/sift-ui/src/pages/instance.rs:470-512 (no navbar) | missing RE-VERIFIED 2026-11-01: a path bar is drawn above the listing with one target per ancestor (instance.rs:715-763); the reference's navbar carries more than the path, and that remainder is still missing. | OPEN-WITH-CAUSE | P0 |
| 157 | files — table header | `packages/ui/src/layouts/shared/files-tab/components/FileTableHeader.vue:2-9` | packages/ui/src/layouts/shared/files-tab/components/FileTableHeader.vue:2-9 (sticky h-12 pl-3 pr-4 bg-surface-3 rounded-t-[20px]) ; :35-90 (Size 100px, Created 160px, Modified 160px, Actions min-51px, columns hidden below 800px) | stuck: rounded-none, border-y, shadow-md (:7); border-radius 100ms (:4) | absent | missing | OPEN | P0 |
| 158 | files — file row | `packages/ui/src/layouts/shared/files-tab/components/FileTableRow.vue:14-68` | packages/ui/src/layouts/shared/files-tab/components/FileTableRow.vue:14-68 (checkbox, 20px icon, name; size/date text-sm secondary; row menu) ; :175-190 (py-3 pl-3 pr-4, border-t surface-4, alt surfaces, rounded-b-[20px] last) | hover bg-surface-2.5 for dirs/editable files (:187); selected bg-surface-2.5; drag source opacity-50; drop target bg-brand-highlight (:179-181); colours 100ms (:188) | crates/sift-ui/src/pages/instance.rs:470-512 (20px rows, 12px name, 11px size) | row height/padding, columns, checkbox, menu and hover all differ | OPEN | P0 |
| 159 | files — list container | `packages/ui/src/layouts/shared/files-tab/layout.vue:59` | packages/ui/src/layouts/shared/files-tab/layout.vue:59 (rounded-[20px] border surface-4 shadow-sm overflow-clip) | — | crates/sift-ui/src/pages/instance.rs:59,306-310 (card radius 12) | radius 12 vs 20 | OPEN | P1 |
| 160 | files — drag row onto folder | `packages/ui/src/layouts/shared/files-tab/composables/file-drag-state.ts:16` | packages/ui/src/layouts/shared/files-tab/composables/file-drag-state.ts:16 (5px threshold), :22-30 (ghost rounded-lg bg-bg-raised p-3 shadow-lg), :94-101 (self and child drops rejected) | ghost follows the pointer; target row bg-brand-highlight (FileTableRow.vue:179-180) | absent | missing | OPEN | P1 |
| 161 | files — drag reorder of rows | `file-drag-state.ts:106-120` | UNKNOWN — the only row drag in the source is row-to-folder move (file-drag-state.ts:106-120); no reorder path exists | — | absent | UNKNOWN; evidence would be a capture of two files swapping order, or a source field holding file order | NOT-IN-SOURCE | P2 |
| 162 | files — multi-select | `packages/ui/src/layouts/shared/files-tab/composables/file-selection.ts:6-51` | packages/ui/src/layouts/shared/files-tab/composables/file-selection.ts:6-51 (per-row Checkbox + select-all, set of paths) | no shift or range select in source (:9-17) | absent | missing | OPEN | P1 |
| 163 | files — selection bar | `packages/ui/src/layouts/shared/files-tab/layout.vue:167-176` | packages/ui/src/layouts/shared/files-tab/layout.vue:167-176 (count 'N selected', Clear), :177-201 (Create ZIP, Delete red) | shown while selection is non-empty; Create ZIP only when zipPaths exist (:179) | absent | missing | OPEN | P1 |
| 164 | files — rename | `packages/ui/src/layouts/shared/files-tab/components/modals/FileRenameItemModal.vue:1-27` | packages/ui/src/layouts/shared/files-tab/components/modals/FileRenameItemModal.vue:1-27 (NewModal max-width 500px, md:min-w-[400px], 'New name' input, Cancel / Rename) | inline validation message (:11) | absent | missing; the source has no in-place rename for files RE-VERIFIED 2026-11-01: `rename_entry` is implemented with four tests (instance.rs:466-488, tests :1120-1151), but nothing calls it — `state.renaming` is threaded into `files_tab` (:206, :346, :684) and never read, so there is still no control. | OPEN-WITH-CAUSE | P1 |
| 165 | files — delete confirm | `packages/ui/src/layouts/shared/files-tab/components/modals/FileDeleteItemModal.vue:1-23` | packages/ui/src/layouts/shared/files-tab/components/modals/FileDeleteItemModal.vue:1-23 (fade=danger, 'Deleting "{name}"', irreversible warning, Cancel / Delete red) | — | absent | missing RE-VERIFIED 2026-11-01: a file row's `x` arms on the first press and removes on the second (instance.rs:782-813), which is the reference's two-step delete. | VERIFIED | P1 |
| 166 | files — move dialog | `packages/ui/src/layouts/shared/files-tab/components/modals/FileMoveItemModal.vue:1-114` | packages/ui/src/layouts/shared/files-tab/components/modals/FileMoveItemModal.vue:1-114 (folder picker aware of current path) | — | absent | missing | OPEN | P2 |
| 167 | files — drop-to-upload overlay | `packages/ui/src/layouts/shared/files-tab/components/upload/FileUploadDragAndDrop.vue:10-27` | packages/ui/src/layouts/shared/files-tab/components/upload/FileUploadDragAndDrop.vue:10-27 (absolute inset-0 bg-black/60 rounded-2xl, 64px UploadIcon, text-xl 'Drop {type} here to upload') | shown while files are dragged over the table | absent | missing RE-VERIFIED 2026-11-01: a drop does land — `drop::copy_into` is called with the directory the panel is showing (instance.rs:689-702, drop.rs:29-53) — but no overlay is drawn while a drag is in progress, which is what this row is about. | OPEN-WITH-CAUSE | P1 |
| 168 | files — empty folder | `packages/ui/src/layouts/shared/files-tab/layout.vue:114-127` | packages/ui/src/layouts/shared/files-tab/layout.vue:114-127 (p-20 centre, 64px FolderOpenIcon, h3 text-2xl bold, p text-sm secondary) | — | crates/sift-ui/src/pages/instance.rs:470-479 (generic stub sentence) | different | OPEN | P2 |
| 169 | files — read-only folders | `apps/app-frontend/src/pages/instance/files/index.vue:69-75` | apps/app-frontend/src/pages/instance/files/index.vue:69-75 (mods/ read-only) ; :32-35 (reason 'Manage your installed content via the Content tab') | write actions disabled with the reason as tooltip | absent | missing | OPEN | P2 |
| 170 | screenshots — page chrome | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1487-1501` | apps/app-frontend/src/components/ui/screenshots-page/index.vue:1487-1501 (flex h-full flex-col gap-3, h1 text-2xl font-bold, toolbar) | empty, error and no-results states (:1520-1544) | crates/sift-ui/src/pages/instance.rs:374-421 (no heading, no toolbar) | missing | OPEN | P1 |
| 171 | screenshots — grid | `apps/app-frontend/src/components/ui/screenshots-page/group.vue:210`<br>`index.vue:127` | apps/app-frontend/src/components/ui/screenshots-page/group.vue:210 (grid-cols-1 sm:grid-cols-2 2xl:grid-cols-4 gap-3) ; index.vue:127 (SCREENSHOT_GRID_GAP = 12) | responsive 1/2/4 columns | crates/sift-ui/src/pages/instance.rs:394-397 (fixed 3 columns, gap 8) | column count and gap wrong | OPEN | P1 |
| 172 | screenshots — card | `apps/app-frontend/src/components/ui/screenshots-page/card.vue:172` | apps/app-frontend/src/components/ui/screenshots-page/card.vue:172 (aspect-video rounded-xl border surface-5 bg-surface-2 shadow-sm), :196-212 (50px hit area, 24px circular selection), :233-243 (footer gradient p-3, name text-sm semibold, date text-xs) | hover brightness-110 (:172); selection 200ms (:206); footer opacity 200ms (:233) | crates/sift-ui/src/pages/instance.rs:397-421 (bare image tiles, radius 6) | missing card chrome, selection and hover RE-VERIFIED 2026-11-01: the hover half exists — a caption bar with the file name and an armed `x` appears under the pointer and nowhere else (instance.rs:413-460) — while the card chrome, the 50-point selection hit area and the 24-point disc do not. | OPEN-WITH-CAUSE | P2 |
| 173 | screenshots — group section header | `apps/app-frontend/src/components/ui/screenshots-page/section.vue:111-148` | apps/app-frontend/src/components/ui/screenshots-page/section.vue:111-148 (h-10, border-b surface-5, chevron, title text-base semibold, count tag) | chevron rotate-180 transition-all 300ms (:125) | absent | missing | OPEN | P2 |
| 174 | worlds — toolbar | `apps/app-frontend/src/pages/instance/worlds/index.vue:24-48`<br>`af/pages/instance/worlds/index.vue:23-125` | apps/app-frontend/src/pages/instance/worlds/index.vue:24-48 (search Input medium flex-1 + Add server outlined lg + Browse servers colored brand lg) ‖ file-management.md row 55 — element "Worlds — List, search, filters, empty state": Singleplayer worlds plus servers, search by name, type and status filters, refresh-all, empty state with `Add server` / `Browse servers` (`af/pages/instance/worlds/index.vue:23-125`, `:660-697`; search text `searchWorldsPlaceholder` `:35`). | — ‖ file-management.md row 55: — (this report has no states column) | crates/sift-ui/src/pages/instance.rs:423-469 (plain rows) ‖ file-management.md row 55: Rows of `(folder, name)` from `saves/`, name read out of `level.dat` (`sift-ui/pages/instance.rs:422-467`, `saves` at `:569-596`), capped at 24; empty state is the shared stub. | missing ‖ file-management.md row 55: Read-only listing; no search, filters or empty-state actions. | OPEN | P0 |
| 175 | worlds — filter / refresh row | `apps/app-frontend/src/pages/instance/worlds/index.vue:49-70` | apps/app-frontend/src/pages/instance/worlds/index.vue:49-70 (FilterPills + Refresh quiet !text-sm) | spinner while refreshing (:65) | absent | missing | OPEN | P1 |
| 176 | worlds — world card | `apps/app-frontend/src/components/ui/world/WorldItem.vue:477-486` | apps/app-frontend/src/components/ui/world/WorldItem.vue:477-486 (grid-cols-[auto_minmax(0,3fr)_minmax(0,4fr)_auto] gap-2 p-3 rounded-[20px] border surface-4 min-h-20 bg-bg-raised) | hover brightness 1.25 (:478) and active scale .985 (:470); highlight animation 4s (:683-722) | crates/sift-ui/src/pages/instance.rs:437-464 (20px rows, 12px name, 11px folder) | layout entirely different | OPEN | P0 |
| 177 | worlds — card avatar | `apps/app-frontend/src/components/ui/world/WorldItem.vue:485-494` | apps/app-frontend/src/components/ui/world/WorldItem.vue:485-494 (Avatar 48px, rounded-[14px], no-shadow) | — | none | missing | OPEN | P1 |
| 178 | worlds — card secondary line | `apps/app-frontend/src/components/ui/world/WorldItem.vue:578-609` | apps/app-frontend/src/components/ui/world/WorldItem.vue:578-609 (instance link + BulletDivider + relative last played, else 'Not played yet') | tooltip with the absolute date (:598) | crates/sift-ui/src/pages/instance.rs:463 (folder name only) | wrong content | OPEN | P1 |
| 179 | worlds — card actions | `apps/app-frontend/src/components/ui/world/WorldItem.vue:643-672` | apps/app-frontend/src/components/ui/world/WorldItem.vue:643-672 (Play colored brand / Stop red md + quiet overflow) | disabled with reasons when quick-play unsupported or another world runs (:304-324) | none | missing | OPEN | P1 |
| 180 | worlds — empty state | `apps/app-frontend/src/pages/instance/worlds/index.vue:109-125` | apps/app-frontend/src/pages/instance/worlds/index.vue:109-125 (EmptyState empty-inbox, heading + description + two buttons) | — | crates/sift-ui/src/pages/instance.rs:427-435 (own sentence) | wrong empty state | OPEN | P1 |
| 181 | logs — page shell | `apps/app-frontend/src/pages/instance/logs/index.vue:2-4`<br>`apps/app-frontend/src/routes.js:157-159` | apps/app-frontend/src/pages/instance/logs/index.vue:2-4 (flex flex-col gap-4 h-full + ConsolePageLayout) ; fixed render mode apps/app-frontend/src/routes.js:157-159 | — | crates/sift-ui/src/pages/instance.rs:513-539 (11px text lines) | no console layout at all | OPEN | P0 |
| 182 | logs — log toolbar | `packages/ui/src/layouts/shared/console/layout.vue:22-40` | packages/ui/src/layouts/shared/console/layout.vue:22-40 (Search logs Input medium clearable + source Combobox w-[220px] trigger-size lg) | live vs historical source switch (logs/index.vue:103-109) | absent | missing | OPEN | P0 |
| 183 | logs — filter pills and actions | `packages/ui/src/layouts/shared/console/layout.vue:42-64` | packages/ui/src/layouts/shared/console/layout.vue:42-64 (ConsoleFilterPills; clear / share / fullscreen / delete) | delete disabled for latest.log while running (logs/index.vue:130-134,158) | absent | missing | OPEN | P1 |
| 184 | logs — terminal | `packages/ui/src/components/base/BaseTerminal.vue:3-16`<br>`packages/ui/src/composables/terminal.ts:173-174` | packages/ui/src/components/base/BaseTerminal.vue:3-16 (bg-surface-2 rounded-[20px] border surface-4, pb-2 pt-1, scroll-to-bottom button bottom-4 right-4) ; packages/ui/src/composables/terminal.ts:173-174 (monospace 14px) | scrollback Infinity (BaseTerminal.vue:60); empty-state bubble written into the terminal (:33-43) | crates/sift-ui/src/pages/instance.rs:513-539 | missing | OPEN | P0 |
| 185 | logs — delete log dialog | `packages/ui/src/layouts/shared/console/layout.vue:82-100` | packages/ui/src/layouts/shared/console/layout.vue:82-100 ('Delete log file', fade=danger, critical admonition, Cancel / Delete) | — | absent | missing | OPEN | P2 |

## 5. Settings

### 5a. Driven against the reference, 2026-11-01 (supersedes the settings rows above that disagree)

Every tab was opened on both clients at 1440x900 and read back with `tools/parity/probe.py`
(OCR for words, colour runs for rectangles). The reference ran on display `:98`, a private X
server: display `:99` is shared with other sessions, and a capture taken there is as likely to
be another session's window as the reference's — one was, which is why this section exists.
Captures: `.scratch/cap/ref98-tab-*.png` and `.scratch/cap/ours98-tab-*.png`.

Coordinates below are window coordinates with the window at 0,0; both clients draw their own
48-point title bar, so a screen capture needs no offset on this display.

| # | element | reference | ours (before) | ours (after) | status |
|---:|---|---|---|---|---|
| S1 | rail rows at rest | 13 rows; `Feature flags` absent (`developerOnly`, `index.vue:211-213`) | 13 rows, and no way to reach `Feature flags` at all | 13 rows; six presses on the footer mark add the fourteenth | DONE |
| S2 | rail row pitch | row tops 246, 290, 334, 378, then ACCOUNT heading 434, Profile 465, Java 639, Resources 853 | 240, 284, 328, 372, 428, 454, 624, 838 | 244, 288, 332, 376, 432, 462, 636, 854 — within 2-3 points everywhere | DONE |
| S3 | rail footer | "Modrinth App 0.22.0" at 733, "Linux 20261025.0.616225" at 753, 24-point mark at 272,730 | same rows, "Sift Launcher 0.1.62" / "Linux 0.1.62" | unchanged; the mark is now the developer-mode button | DONE |
| S4 | rail scrollbar | 4-point `--surface-5` thumb, 321 points in a 481-point pane, moving with the scroll | a fixed rectangle at the capture's coordinates that never moved | computed from the scroll offset and the content height | DONE |
| S5 | Appearance | heading 231, description 259, card labels 420, sync row 653 | 231, 257, 419, 651 | unchanged (within 2) | DONE |
| S6 | Features | Instances 348, Worlds 392, Files 444, Screenshots 496, Sidebar 580 | 344, 384, 431, 479, 560 | 350, 394, 446, 498, 582 | DONE |
| S7 | Behavior | rule 316, Startup and navigation 348, Minimize app 391, Home and content 647 | no rule; 311, 350, 591 | rule 316, 350, 393, 652 | DONE |
| S8 | Language | 20-point heading 231; a bordered orange callout 261..352; description 373; search field 424; "Standard languages" 473; rows from 487 at 44 pitch | the warning as the heading's sub-line; an invented `Select` on the category heading holding five of the locales; a search field that filtered nothing; 26-point rows | callout, description, search, category and 40-point bordered rows; the search filters; no select | DONE |
| S9 | Profile, signed out | `EmptyState`: "Modrinth account required" 470, one line 505, brand button 545..591 | the whole signed-in form: upload, remove, username, bio | the empty state, with the button saying a Modrinth account is unavailable in this build | DONE (text) |
| S10 | Social, signed out | same empty state, "You control who can interact with you, and manage blocked users with a Modrinth Account." | three chip rows and a blocked-users section | the empty state | DONE (text) |
| S11 | Privacy | heading 231, "Manage preferences" 272, ads paragraph 309, Telemetry 399 | paragraph above the button: 231, 343, 261 | 231, 276, 313 | DONE |
| S12 | Java and memory | four blocks at 275, 433, 591, 749; field 36 tall; buttons 34 tall under a 10-point gap | 275, 405, 535, 665 | 275, 433, 591, 749 | DONE |
| S13 | Troubleshooting | each section is heading, button, description: 231, 274, 312, then 379 | heading carried the description, so the button and paragraph were swapped | 231, 282, 319 | DONE |
| S14 | Resources | heading 231 with a "Calculating" status at the right; description 259; bar, legend, then the two buttons side by side at 332 (the destructive one red, #8e3448) | description, two stacked grey buttons, then the bar | bar, legend, then both buttons in one row, the destructive one red | DONE |
| S15 | Syncing | rows at 297/363/429/495 with an edit IconButton beside each toggle; 16-point row gap | rows at 306/381/456/531 with a toggle only; 24-point gap | rows at 298/365/432/499; the gap is 16 on this tab and 24 elsewhere | OPEN — the edit IconButton opens a per-option modal this build does not have (BLOCKED-SERVICE) |
| S16 | Profile and Social illustration | the Rinthbot artwork, 200 points tall above the title | absent; its space is kept so the words land where the capture puts them | absent | OPEN — no artwork ships in this tree |
| S17 | Language locale list | ~30 locales with flag, translated name and coverage share (100%, 7%, 87%, ...) | five names, English at 100% and the rest at 0% | five names, same figures | OPEN — the coverage figure is what *this* build carries, which is the row's stated purpose; adding the reference's share would promise translations that are not here |
| S18 | Theme cards | 3-up grid of ~199-point cards, label row with a sun or moon marker | 198x116 cards with a 42-point footer, markers drawn | unchanged | DONE |
| S19 | Storage legend | columns with a size-3 dot, a 16-point label and a semibold value, three to a row; the total is not printed | one inline line of 12-point text | three columns with dots and semibold values, the total above the bar's right end | DONE (the reference's "Calculating" status has no counterpart: this launcher measures before the first frame) |
| S20 | Heading gaps | `mt-4` (16) above a group of rows, `gap-2.5` (10) inside a section that stacks a heading, a control and a paragraph | one 16-point gap everywhere | the gap follows the next block | DONE |
| S21 | Row line box | 28 points for both `text-lg` titles and `text-xl` headings, which is what makes a description-less row pitch at 52 | the galley's own height, so rows pitched at 47 | 28-point line box | DONE |
| S22 | Text fields | `w-full` in every standalone field (532..1157 in the language tab) and 36 points tall | 200 points wide, 28 tall | full width, 36 tall | DONE |
| S23 | Per-version Java buttons | Install recommended / Detect / Browse under each version's field | drew and reported nothing that could be seen; the answer went to a line the tab never drew | Browse opens the folder picker, Detect reads the runtime, Install recommended says the download service is missing — and the answer is drawn under the buttons | DONE |
| S24 | Resources and Troubleshooting buttons | each acts and reports | all inert: "Verify and repair", "Clear unused", "Open backups folder", "Export debug info", "Manage preferences" | backups folder opens, debug info is written to `logs/debug-info.txt`, verify reads every instance's profile, and the two that cannot act say so under themselves | DONE |
| S25 | Per-row reset | a quiet Button on feature-flag rows only (`feature-flag-settings.vue:63-69`) | a "Reset to default" pill on every changed row of every tab | feature-flag rows only | DONE |
| S26 | Modal plate | 256..1183 x 115..788 at 1440x900 | 256..1184 x 114..786 | unchanged | DONE (within 2 points) |
| S27 | Click outside to close | a press on the scrim calls `hide()` (`NewModal.vue:35,222`) | only the close button and Escape closed it | a press anywhere outside the plate closes it, except on the frame that opens it | DONE (landed inside commit 9114a1c, which was another session's) |
| S28 | Every control driven | — | — | driven and captured: the Worlds switch (track #1bd96a to #34363c, `ours98-switch-*`), the quick-instance slider (20 to 19, `ours98-slider-*`), the pre-launch field (typed `echo hi`, stored in `app-settings.json`), the language list (`deu` leaves only Deutsch, `port` only Português (Brasil)), a theme card (the whole window changes), the Java Browse button (the folder picker opens on `java-runtime-epsilon`), the body's wheel (`ours98-features-scrolled`), Clear cache's two-step answer (`ours98-clearcache-*`), Open backups folder and Export debug info (`ours98-backups`, `ours98-export`, and `logs/debug-info.txt` on disk) | DONE |
| S29 | Settings survive a restart | the reference writes `app-settings.json` | ours writes it on close | Escape closed the modal, the file gained `"standard languages": "Deutsch"`, and reopening painted the ring on that row | DONE |
| S30 | Language choice across a restart | the reference stores `locale` in its settings file | the choice was written and then dropped on load: the loader validated every stored choice against a `Control::Select`'s options, and this tab's choice is a radio list, so the ring came back on English with nothing said | `select_options` answers for a `Block::RadioList` too, and a test writes Deutsch, reads it back and asserts the list it is validated against is the one the tab draws | DONE |
| S31 | Sliders on a line of their own | `Slider.vue` is a full-width track with min and max labels and a number field beside it (Java memory, maximum downloads, maximum writes) | drew the track and the knob, and no press moved them: the drag handler only existed for a slider inside a row | the three standalone sliders are draggable, keyed by the section heading above them so the value persists (memory read 512 MB, then 20722 MB after a drag) | DONE (the full-width track, the min and max labels and the number field are still open: rows 227-229 above) |
| S32 | Java and memory, lower half | `index.vue:76-136`: a `border-t pt-6` group of three sections, each a heading, a control and the line under it; no tab-level Detect or Browse | a tab-level Detect and Browse pair that the source does not have, and the three descriptions on the headings instead of under their controls | the pair is gone (each version already carries its own), and the descriptions sit under their sliders and fields | DONE |
| S33 | Window tab's width and height | two `type="number"` inputs bound to the instance defaults, opening on 854 and 480; both disabled while Fullscreen is on | two text fields whose placeholder read "Enter width…" and "Enter height…", with no value | number inputs opening on 854 and 480 | DONE (the disable-while-fullscreen link between the rows is still open: it needs one row to read another's switch) |


| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 186 | modal shell — plate width, rail vs tab strip | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:380-381`<br>`packages/ui/src/components/modal/NewModal.vue:743`<br>`packages/ui/src/components/modal/TabbedModal.vue:148` | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:380-381` width="928px" max-width="928px", capped by `packages/ui/src/components/modal/NewModal.vue:743` max-width min(928px, 100% - 2*var(--gap-lg)); the body is a two-column grid, `packages/ui/src/components/modal/TabbedModal.vue:148` grid-cols-[250px_minmax(0,1fr)] | static | `crates/sift-ui/src/pages/settings.rs:901` DIALOG_WIDTH 928, `:2503` clamps to screen - 2*DIALOG_MARGIN | none - rail, not tab strip, and the width matches | DONE | P0 |
| 187 | modal shell — plate height | `packages/ui/src/components/modal/TabbedModal.vue:148`<br>`packages/ui/src/components/modal/NewModal.vue:50`<br>`apps/app-frontend/src/features/settings/app-settings-modal/index.vue:387` | `packages/ui/src/components/modal/TabbedModal.vue:148` h-[min(65vh,640px)] under an 80pt header (`packages/ui/src/components/modal/NewModal.vue:50` p-6 + `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:387` text-2xl) | static | `crates/sift-ui/src/pages/settings.rs:905` 0.65, `:908` 640, `:939` HEADER_HEIGHT 80 | none - 80 + min(65% of the window, 640) | DONE | P0 |
| 188 | modal shell — plate corner | `packages/ui/src/components/modal/TabbedModal.vue:142`<br>`packages/assets/styles/variables.scss:205` | `packages/ui/src/components/modal/TabbedModal.vue:142` !rounded-[20px] = --radius-xl (`packages/assets/styles/variables.scss:205`) | static | `crates/sift-ui/src/pages/settings.rs:916` PLATE_RADIUS = theme::RADIUS_XL (`crates/sift-ui/src/theme.rs:800` 20) | none | DONE | P0 |
| 189 | modal shell — plate border and drop shadow | `packages/ui/src/components/modal/NewModal.vue:44`<br>`packages/ui/src/components/modal/NewModal.vue:741` | `packages/ui/src/components/modal/NewModal.vue:44` bg-bg-raised border-surface-5; `packages/ui/src/components/modal/NewModal.vue:741` box-shadow 4px 4px 26px 10px rgba(0,0,0,0.08) | static | `crates/sift-ui/src/pages/settings.rs:2118-2124` fill RAISED_BG + 1pt DIVIDER stroke | no drop shadow, so the plate edge only separates from the near-opaque scrim by its border ⇒ renderer risk: blur — 4px 4px 26px 10px drop shadow | RENDERER-RISK | P1 |
| 190 | modal shell — scrim | `packages/ui/src/components/modal/NewModal.vue:679`<br>`packages/ui/src/components/modal/NewModal.vue:698`<br>`packages/ui/src/components/modal/NewModal.vue:664`<br>`packages/ui/src/components/modal/NewModal.vue:662-668` | `packages/ui/src/components/modal/NewModal.vue:679` linear-gradient rgba(29,48,43,0.52) to rgba(14,21,26,0.95); `packages/ui/src/components/modal/NewModal.vue:698` backdrop-filter blur(5px); `packages/ui/src/components/modal/NewModal.vue:664` inset -5rem, radius 180px ‖ motion.md row 21 — element "modal — settings and about open/close": `packages/ui/src/components/modal/NewModal.vue:662-668` overlay opacity 0 to 1 on `transition: all 0.2s ease-out`; `:694-699` shown adds `backdrop-filter: blur(5px)` | opacity 0 to 1 by `packages/ui/src/components/modal/NewModal.vue:668` transition all 0.2s ease-out; off under prefers-reduced-motion (`:690`) ‖ motion.md row 21: 200ms ease-out both directions; trigger = the open flag | `crates/sift-ui/src/pages/settings.rs:1081` SCRIM_ALPHA 140 at `:2117` flat from_black_alpha ‖ motion.md row 21: `crates/sift-ui/src/shell.rs:400-406` | flat 0.549 black instead of the teal-to-navy gradient with a 5px backdrop blur, and no fade ‖ motion.md row 21: dialog appears the frame the flag flips; no dim, no blur, no 200ms ramp ⇒ renderer risk: blur — backdrop-filter blur(5px) over a two-stop gradient scrim | RENDERER-RISK | P0 |
| 191 | modal shell — enter animation | `packages/ui/src/components/modal/NewModal.vue:476-481`<br>`packages/ui/src/components/modal/NewModal.vue:748`<br>`packages/ui/src/components/modal/NewModal.vue:752`<br>`packages/ui/src/components/modal/NewModal.vue:748-752` | `packages/ui/src/components/modal/NewModal.vue:476-481` visible is set 50ms after open; `packages/ui/src/components/modal/NewModal.vue:748` scale 0.97 to 1 and `packages/ui/src/components/modal/NewModal.vue:752` transition all 0.2s ease-in-out ‖ motion.md row 22 — element "modal — modal body": `packages/ui/src/components/modal/NewModal.vue:748-752` body at scale 0.97 and opacity 0 on `transition: all 0.2s ease-in-out`; `:730-736` shown sets scale 1 and opacity 1 | trigger: show(); container resolves its mouse-offset translate with 0.2s ease-out (`packages/ui/src/components/modal/NewModal.vue:729-730`) ‖ motion.md row 22: 200ms ease-in-out scale plus fade, both directions | `crates/sift-ui/src/pages/settings.rs:2099-2111` draws at full size and full opacity on the first frame ‖ motion.md row 22: none in `crates/sift-ui/src` (`crates/sift-ui/src/pages/settings.rs` paints the plate directly) | no 50ms beat, no scale, no fade ‖ motion.md row 22: no scale, no fade, no per-frame progress | OPEN | P0 |
| 192 | modal shell — exit animation | `packages/ui/src/components/modal/NewModal.vue:498-501` | `packages/ui/src/components/modal/NewModal.vue:498-501` unmounts 300ms after hide (200ms transition + slack) | trigger: hide(), Escape, overlay click | `crates/sift-ui/src/pages/settings.rs:2421` and `:2424-2426` set open=false, gone the next frame | the plate vanishes instead of shrinking and fading out | OPEN | P1 |
| 193 | modal shell — open parallax | `packages/ui/src/components/modal/NewModal.vue:724` | `packages/ui/src/components/modal/NewModal.vue:724` container transform translate(var(--modal-mouse-x-offset), var(--modal-mouse-y-offset)); `:574-575` each is (-50vw + mouseX)/16; `:729` resolves to translate(0,0) when shown | trigger: pointer position at show() | not implemented | no mouse-anchored entry offset | OPEN | P2 |
| 194 | modal shell — click outside to close | `packages/ui/src/components/modal/NewModal.vue:222`<br>`packages/ui/src/components/modal/NewModal.vue:35`<br>`UI/components/modal/NewModal.vue:35` | `packages/ui/src/components/modal/NewModal.vue:222` closeOnClickOutside defaults true; `packages/ui/src/components/modal/NewModal.vue:35` the overlay click calls hide() ‖ interaction.md row 70 — element "click-outside — a press on a modal's backdrop closes it": `UI/components/modal/NewModal.vue:35` (`@click` on `.modal-overlay`, gated by `closeOnClickOutside` default `true` `:222`), with a 4 px drag threshold on the tauri overlay `:247-263` | trigger: click on the scrim ‖ interaction.md row 70: — (this report has no states column) | `crates/sift-ui/src/pages/settings.rs:2417-2422` only the close button is interactive ‖ interaction.md row 70: About does it: `SRC/pages/about.rs:160, 198-203` (with the plate swallowing presses `:175`). Settings does **not**: `SRC/pages/settings.rs:2407-2426` handles only rail rows, the close box and `Escape` | a click on the scrim does nothing ‖ interaction.md row 70: the settings modal cannot be dismissed by clicking outside, where both reference modals can | OPEN | P0 |
| 195 | modal shell — Escape to close | `packages/ui/src/components/modal/NewModal.vue:602-604`<br>`UI/components/modal/NewModal.vue:602` | `packages/ui/src/components/modal/NewModal.vue:602-604` closeOnEsc and closable and `:593` isTopmostModal ‖ interaction.md row 4 — element "keyboard — `Escape` closes a modal": `UI/components/modal/NewModal.vue:602`, defaults `closeOnEsc: true` at `:223` | trigger: keydown Escape ‖ interaction.md row 4: — (this report has no states column) | `crates/sift-ui/src/pages/settings.rs:2424-2426` unconditional ‖ interaction.md row 4: About: `SRC/pages/about.rs:140-143`; Settings: `SRC/pages/settings.rs:2424-2426` | closes even while an open select list or the folder picker owns the press, and with no topmost check ‖ interaction.md row 4: matches; but Settings' folder picker (`SRC/pages/settings.rs:2202`) has no `Escape` of its own, so one `Escape` closes the whole modal rather than the picker — which of the two should win is **not stated in source** | OPEN | P0 |
| 196 | modal shell — focus trap | `packages/ui/src/components/modal/NewModal.vue:597-599`<br>`UI/components/modal/NewModal.vue:597-598, 607-640` | `packages/ui/src/components/modal/NewModal.vue:597-599` Tab is intercepted; `:607-640` wraps first and last over `:355-356` FOCUSABLE_SELECTOR ‖ interaction.md row 11 — element "keyboard — `Tab` is trapped inside a modal": `UI/components/modal/NewModal.vue:597-598, 607-640` over `FOCUSABLE_SELECTOR` `:355-356`, wrapping both ways, plus `getFocusableElements` visibility filter `:381-390` | trigger: keydown Tab ‖ interaction.md row 11: — (this report has no states column) | not implemented ‖ interaction.md row 11: About and Settings take no focus: `SRC/pages/about.rs:160-203`, `SRC/pages/settings.rs:2407-2426` are pointer targets only | Tab walks out of the dialog; no wrap ‖ interaction.md row 11: a modal can be tabbed past into the page under it — except nothing is focusable, so Tab does nothing at all | OPEN | P0 |
| 197 | modal shell — initial focus and focus restore | `packages/ui/src/components/modal/NewModal.vue:405-435`<br>`UI/components/modal/NewModal.vue:400-435` | `packages/ui/src/components/modal/NewModal.vue:405-435` focusModal(); `:460` remembers the active element; `:494-497` restores it on hide ‖ interaction.md row 54 — element "focus — a modal takes focus on open and restores the previous element on close": `UI/components/modal/NewModal.vue:400-435` (`focusModal`, `initialFocus`), scheduling `:437-442`, capture `:460`, restore `:494-496` | trigger: open / close ‖ interaction.md row 54: — (this report has no states column) | not implemented ‖ interaction.md row 54: `SRC/pages/about.rs:134-203` and `SRC/pages/settings.rs:2395-2426` never move focus, so there is nothing to restore | nothing is focused on open and focus is not returned on close ‖ interaction.md row 54: absent | OPEN | P1 |
| 198 | modal shell — dialog semantics | `packages/ui/src/components/modal/NewModal.vue:40-43` | `packages/ui/src/components/modal/NewModal.vue:40-43` role="dialog" aria-modal="true" tabindex="-1" aria-labelledby=headerId | static | not implemented | no role, no accessible name | OPEN | P1 |
| 199 | modal shell — unsaved-changes guard | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:382-384`<br>`packages/ui/src/components/modal/TabbedModal.vue:101` | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:382-384` before-hide and before-tab-change = canLeaveCurrentTab (`:232-241`) which nudges the popup instead of closing; `packages/ui/src/components/modal/TabbedModal.vue:101` blocks setTab | trigger: close, tab change, Escape while a tab has changes | `crates/sift-ui/src/pages/settings.rs:2487-2492` writes silently when open goes false | edits are committed with no Save and no prompt; a tab change abandons them silently | OPEN | P0 |
| 200 | header — title | `packages/ui/src/components/modal/NewModal.vue:54`<br>`apps/app-frontend/src/features/settings/app-settings-modal/index.vue:386-390` | `packages/ui/src/components/modal/NewModal.vue:54` text-2xl font-semibold text-contrast; text from `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:386-390` | static | `crates/sift-ui/src/pages/settings.rs:930-935` TITLE_SIZE 24 on a 32 line, `:2644-2650` | none | DONE | P0 |
| 201 | header — header rule | `packages/ui/src/components/modal/NewModal.vue:50` | `packages/ui/src/components/modal/NewModal.vue:50` border-b 1px border-surface-5 across the full header width, clipped only by the plate radius | static | `crates/sift-ui/src/pages/settings.rs:2652-2659` hline inset by PLATE_RADIUS at both ends | the rule is 40px narrower than the reference's, so it floats instead of touching the plate edges | OPEN | P0 |
| 202 | header — close button | `packages/ui/src/components/modal/NewModal.vue:61-69`<br>`packages/ui/src/components/base/buttons/ButtonFrame.vue:39`<br>`packages/ui/src/components/base/buttons/ButtonFrame.vue:135-140` | `packages/ui/src/components/modal/NewModal.vue:61-69` IconButton md circular, 36x36 (`packages/ui/src/components/base/buttons/ButtonFrame.vue:39`), type base = bg-surface-4 plus inset 0 0 0 1px surface-5 and 0 1px 1px (`packages/ui/src/components/base/buttons/ButtonFrame.vue:135-140`) | hover brightness var(--hover-brightness) (`packages/ui/src/components/base/buttons/ButtonFrame.vue:21`), active scale 0.97 (`:19`), 150ms ease-out (`:18`) | `crates/sift-ui/src/pages/settings.rs:1073` CLOSE_SIZE 36, `:2662` fill SUPER_RAISED_BG | the 1pt ring and soft shadow are missing, and the button has no hover or press feedback | OPEN | P1 |
| 203 | rail — column width and padding | `packages/ui/src/components/modal/TabbedModal.vue:148` | `packages/ui/src/components/modal/TabbedModal.vue:148` 250px column; `:149` p-4 | static | `crates/sift-ui/src/pages/settings.rs:946` 250, `:949` 16 | none | DONE | P0 |
| 204 | rail — rail rule | `packages/ui/src/components/modal/TabbedModal.vue:152` | `packages/ui/src/components/modal/TabbedModal.vue:152` border-r surface-5 over the rail's full height | static | `crates/sift-ui/src/pages/settings.rs:2683-2687` | none | DONE | P0 |
| 205 | rail — category headings | `packages/ui/src/components/modal/TabbedModal.vue:179` | `packages/ui/src/components/modal/TabbedModal.vue:179` pb-1.5 text-sm font-extrabold uppercase leading-5 text-secondary; `:180` mt-3 when not the first | static | `crates/sift-ui/src/pages/settings.rs:976-986` 20 + 6 line and pad, `:2691-2697` fonts::semibold(14) | not uppercased and set at 600 where the reference is 800 - every group label reads "Display" instead of "DISPLAY" | OPEN | P0 |
| 206 | rail — row box | `packages/ui/src/components/modal/TabbedModal.vue:189` | `packages/ui/src/components/modal/TabbedModal.vue:189` rounded-[14px] px-4 py-2.5 text-base leading-5 font-semibold; `:192` icon size-5, `:189` gap-2 | static | `crates/sift-ui/src/pages/settings.rs:956` 40 tall, `:960` radius 14, `:963` 16 pad, `:972` 16 text, `:966` 20 icon | none when the pane allows the reference pitch | DONE | P0 |
| 207 | rail — row pitch at 900 tall | `packages/ui/src/components/modal/TabbedModal.vue:173` | `packages/ui/src/components/modal/TabbedModal.vue:173` gap-1 (4) around a 40pt row = 44pt pitch; 13 rows plus 4 headings and 3 mt-3 gaps want 708pt against a 553pt pane, so the rail scrolls | scroll: `packages/ui/src/components/modal/TabbedModal.vue:294-307` 4px surface-5 thumb, `:165-168` and `:219-222` top 16 and bottom 56 gradients fading in and out over 200ms (`:158`) | `crates/sift-ui/src/pages/settings.rs:2589-2608` compresses rows to 30pt with 0 gap; no scrollbar, no fades | rows are 32% tighter than the reference, with no scrollbar and no fade, so the rail looks like a different list | OPEN | P0 |
| 208 | rail — row count at rest | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:211-213`<br>`apps/app-frontend/src/composables/use-app-settings.ts:59` | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:211-213` filters developerOnly tabs and `apps/app-frontend/src/composables/use-app-settings.ts:59` starts devMode false, so 13 rows | hidden tab revealed by 5 clicks on the Modrinth glyph (`apps/app-frontend/src/features/settings/app-settings-modal/index.vue:348-355`) | `crates/sift-ui/src/pages/settings.rs:514-529` has 14 rows always and no dev-mode affordance | Feature flags is on screen when the reference hides it | OPEN | P0 |
| 209 | rail — selected row | `packages/ui/src/components/modal/TabbedModal.vue:189` | `packages/ui/src/components/modal/TabbedModal.vue:189` bg-[color-mix(in_srgb,var(--color-brand)_30%,transparent)] text-brand | static while selected; the others keep hover | `crates/sift-ui/src/pages/settings.rs:2704` theme.BUTTON_BG_SELECTED (`crates/sift-ui/src/theme.rs:484` dark, `:657` light solid #00af5c) and `:2707` BUTTON_TEXT_SELECTED (`:659` light white) | in the light theme the selected row is a solid green block with white text instead of a 30% wash with brand text | OPEN | P1 |
| 210 | rail — row hover and press | `packages/ui/src/components/modal/TabbedModal.vue:189` | `packages/ui/src/components/modal/TabbedModal.vue:189` hover:bg-surface-4 hover:text-contrast and active:scale-[0.97] with transition-all | trigger: pointer over, and press | `crates/sift-ui/src/pages/settings.rs:2406-2415` Sense::click only | no hover fill, no hover ink, no press squish | OPEN | P1 |
| 211 | rail — Language badge | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:125`<br>`packages/ui/src/utils/common-messages.ts:27-29`<br>`packages/ui/src/components/modal/TabbedModal.vue:200-205` | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:125` badge = commonMessages.beta (`packages/ui/src/utils/common-messages.ts:27-29` "Beta"), pill per `packages/ui/src/components/modal/TabbedModal.vue:200-205` rounded-full px-1.5 py-0.5 text-xs font-bold bg-brand-highlight text-brand-green | static | not implemented | no Beta pill beside Language | OPEN | P0 |
| 212 | rail — rail footer | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:402-440` | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:402-440` the sidebar footer: Modrinth glyph button (w-6 h-6), "Modrinth App {version}", then platform and OS version, text-sm text-secondary, mb-3 | developer-mode line appears in brand colour when on (`apps/app-frontend/src/features/settings/app-settings-modal/index.vue:412-414`) | not implemented | the rail has no version or OS block, so its bottom is empty where the reference is labelled | OPEN | P0 |
| 213 | rail — truncated label tooltip | `packages/ui/src/components/modal/TabbedModal.vue:195` | `packages/ui/src/components/modal/TabbedModal.vue:195` v-tooltip=truncatedTooltip on each tab label | trigger: label overflows its 250px row | `crates/sift-ui/src/pages/settings.rs:2188-2200` draws tooltips only for standalone buttons | a truncated tab name gives no tooltip | OPEN | P2 |
| 214 | content pane — pane surface | `packages/ui/src/components/modal/TabbedModal.vue:228` | `packages/ui/src/components/modal/TabbedModal.vue:228` bg-surface-2 while header and rail are surface-3 (`:142`) | static | `crates/sift-ui/src/pages/settings.rs:3008` SURFACE_2 | none | DONE | P0 |
| 215 | content pane — content padding | `packages/ui/src/components/modal/TabbedModal.vue:249` | `packages/ui/src/components/modal/TabbedModal.vue:249` px-6 pt-6 with paddingBottom 1.5rem, or bar height + 2.25rem when the bar shows (`:84-86`) | static | `crates/sift-ui/src/pages/settings.rs:989` 24, `:3013` | none while no bar shows | DONE | P0 |
| 216 | content pane — content scroll and fades | `packages/ui/src/components/modal/TabbedModal.vue:245`<br>`packages/ui/src/components/modal/TabbedModal.vue:299-307` | `packages/ui/src/components/modal/TabbedModal.vue:245` overflow-y-auto overscroll-contain, `packages/ui/src/components/modal/TabbedModal.vue:299-307` 4px surface-5 thumb; bottom fade h-14 while scrollable (`:266-278`), top fade h-4 once scrolled (`:229-241`) | fades in and out over 200ms (`packages/ui/src/components/modal/TabbedModal.vue:267`) | `crates/sift-ui/src/pages/settings.rs:2178-2182` wheel only | no scrollbar, no fade, and no scroll affordance at all | OPEN | P1 |
| 217 | content pane — section heading and its description | `packages/ui/src/layouts/shared/appearance-settings/layout.vue:35-42` | `packages/ui/src/layouts/shared/appearance-settings/layout.vue:35-42` flex flex-col gap-1 with h2 text-xl font-semibold text-contrast and p at the inherited 16px | static | `crates/sift-ui/src/pages/settings.rs:997-998` 20 heading, description at ROW_DESCRIPTION_SIZE 12 (`:1012`) | heading matches; its description is 12 not 16 | OPEN | P0 |
| 218 | content pane — row title | `packages/ui/src/layouts/shared/appearance-settings/components/AppearanceSettingRow.vue:25`<br>`apps/app-frontend/src/features/settings/display/behavior-settings.vue:275` | `packages/ui/src/layouts/shared/appearance-settings/components/AppearanceSettingRow.vue:25` text-lg (18) font-semibold text-contrast; same in `apps/app-frontend/src/features/settings/display/behavior-settings.vue:275` | static | `crates/sift-ui/src/pages/settings.rs:1004-1008` ROW_LABEL_SIZE 14, a size the file's own comment records as a deviation | every row title on all 14 tabs is 4px (22%) small | OPEN | P0 |
| 219 | content pane — row description | `packages/ui/src/layouts/shared/appearance-settings/components/AppearanceSettingRow.vue:29`<br>`packages/assets/styles/defaults.scss:17` | `packages/ui/src/layouts/shared/appearance-settings/components/AppearanceSettingRow.vue:29` m-0 mt-1 at the inherited 16px (`packages/assets/styles/defaults.scss:17` font-size 16px), text-secondary | static | `crates/sift-ui/src/pages/settings.rs:1010-1015` 12 with a 4pt gap | descriptions are 25% small, so every row is shorter and wraps sooner than the reference | OPEN | P0 |
| 220 | content pane — row rhythm | `packages/ui/src/layouts/shared/appearance-settings/components/AppearanceSettingRow.vue:20`<br>`packages/ui/src/layouts/shared/appearance-settings/layout.vue:101`<br>`apps/app-frontend/src/features/settings/display/features-settings.vue:283` | `packages/ui/src/layouts/shared/appearance-settings/components/AppearanceSettingRow.vue:20` gap-4 between text and control; rows in gap-6 (`packages/ui/src/layouts/shared/appearance-settings/layout.vue:101`); heading to rows mt-4 (`apps/app-frontend/src/features/settings/display/features-settings.vue:283`) | static | `crates/sift-ui/src/pages/settings.rs:1018` 24, `:1021` 16, `:1025` 16 | none | DONE | P0 |
| 221 | content pane — section rule | `packages/ui/src/layouts/shared/appearance-settings/layout.vue:100` | `packages/ui/src/layouts/shared/appearance-settings/layout.vue:100` mt-8 border-t border-divider pt-6 | static | `crates/sift-ui/src/pages/settings.rs:991-995` 32 and 24 | none | DONE | P0 |
| 222 | content pane — floating unsaved-changes bar | `packages/ui/src/components/modal/TabbedModal.vue:280-288`<br>`packages/ui/src/components/base/FloatingActionBar.vue:238`<br>`packages/ui/src/components/base/UnsavedChangesPopup.vue:86-111` | `packages/ui/src/components/modal/TabbedModal.vue:280-288` absolute bottom-3 left-6 right-6 z-20; toolbar `packages/ui/src/components/base/FloatingActionBar.vue:238` rounded-[20px] bg-surface-3 border-surface-5 px-3 py-2.5 with a shadow; text plus Reset and Save (`packages/ui/src/components/base/UnsavedChangesPopup.vue:86-111`) | enters over 250ms cubic-bezier(0.15,1.4,0.64,0.96) from scale(0.5) translateY(10rem) (`packages/ui/src/components/base/FloatingActionBar.vue:287-302`); nudge shake 300ms ease-in-out (`packages/ui/src/components/base/FloatingActionBar.vue:267-285`) | not implemented | the modal has no Save or Reset at all, so the reference's primary commit path is absent | OPEN | P0 |
| 223 | switch — track | `packages/ui/src/components/base/Toggle.vue:8`<br>`packages/ui/src/components/base/Toggle.vue:10` | `packages/ui/src/components/base/Toggle.vue:8` border 1px surface-5; `packages/ui/src/components/base/Toggle.vue:10` h-6 w-48; `:11` bg-brand when on, bg-button-bg when off ‖ motion.md row 31 — element "controls — Toggle": `packages/ui/src/components/base/Toggle.vue:8` track `transition-[background-color,border-color,opacity] duration-200`, `:17` knob `transition-[transform,background-color,width,height,margin] duration-200`, `:28-29` hover widens the knob to 18px and active squashes it to 14px with a 1px margin change, `:21-23` knob slides `translate-x-[24px]` and turns `bg-black/90` | background-color, border-color and opacity over 200ms (`packages/ui/src/components/base/Toggle.vue:8`) ‖ motion.md row 31: 200ms on every state change; hover and press resize the knob as well as move it | `crates/sift-ui/src/pages/settings.rs:1056-1063` 48x24 pill, `:3937-3946` fill only ‖ motion.md row 31: `crates/sift-ui/src/pages/settings.rs:3937-3953` and `:3956-3973` | no 1px surface-5 ring and no 200ms state transition ‖ motion.md row 31: knob teleports between ends in one frame; no 200ms slide, no hover widen, no press squash | OPEN | P1 |
| 224 | switch — knob | `packages/ui/src/components/base/Toggle.vue:19`<br>`packages/ui/src/components/base/Toggle.vue:22-24` | `packages/ui/src/components/base/Toggle.vue:19` w-4 h-4; `packages/ui/src/components/base/Toggle.vue:22-24` translate-x-[24px] and bg-black/90 when on, bg-secondary when off | transform, background, width, height and margin over 200ms (`packages/ui/src/components/base/Toggle.vue:17`) | `crates/sift-ui/src/pages/settings.rs:3948-3953` white knob when on | the lit knob is white where the reference's is rgba(0,0,0,0.9) - every switch on every tab reads inverted | OPEN | P0 |
| 225 | switch — knob hover and press | `packages/ui/src/components/base/Toggle.vue:28-29` | `packages/ui/src/components/base/Toggle.vue:28-29` hover grows to 18x18 with m-[-1px], press shrinks to 14x14 with m-[1px] | trigger: hover / press | not implemented | no growth on hover, no squish on press | OPEN | P1 |
| 226 | switch — disabled state | `packages/ui/src/components/base/Toggle.vue:12`<br>`packages/ui/src/layouts/shared/appearance-settings/layout.vue:68-71` | `packages/ui/src/components/base/Toggle.vue:12` opacity-50 cursor-not-allowed; drawn off and disabled when signed out (`packages/ui/src/layouts/shared/appearance-settings/layout.vue:68-71`) | static while signed out, with a tooltip (`packages/ui/src/layouts/shared/appearance-settings/layout.vue:63-66`) | `crates/sift-ui/src/pages/settings.rs:1327` draws a normal off switch | a disabled sync switch is indistinguishable from an off one, and gives no reason on hover ⇒ blocked on the account service — the sync switch is drawn off and disabled while signed out | BLOCKED-SERVICE | P1 |
| 227 | slider — geometry | `packages/ui/src/components/base/Slider.vue:2`<br>`packages/ui/src/components/base/Slider.vue:143` | `packages/ui/src/components/base/Slider.vue:2` flex w-full gap-4; `packages/ui/src/components/base/Slider.vue:143` h-10; `:60-68` 6px rounded track with a 32x10 pill thumb offset top-[-13px] right-[-5px]; `:3-9` and `:73-79` min and max labels at text-secondary; `:81-96` a number Input after the track | track fill and thumb travel over 250ms var(--ease-out-expo) (`packages/ui/src/components/base/Slider.vue:300-305`) | `crates/sift-ui/src/pages/settings.rs:1233-1238` 160 wide, 4pt track, 14pt round knob; `:3835-3868` one value label left of the track | 4px track and a round dot instead of 6px and a pill; no min or max label and no number field, so the slider cannot be read or typed into | OPEN | P0 |
| 228 | slider — snap points | `packages/ui/src/components/base/Slider.vue:37-57` | `packages/ui/src/components/base/Slider.vue:37-57` h-[18px] w-1.5 rounded pills at each snap point, with a brand copy clipped at the current value | clip-path over 250ms (`packages/ui/src/components/base/Slider.vue:307-312`) | not implemented | the memory and quick-instance sliders lose their step marks | OPEN | P1 |
| 229 | slider — focus and hover | `packages/ui/src/components/base/Slider.vue:274-277` | `packages/ui/src/components/base/Slider.vue:274-277` focus-visible 3px outline var(--color-focus-ring) offset 3px; `:279-285` hover brightness var(--hover-brightness) on the filled track | trigger: focus / hover | not implemented | no focus ring and no hover brighten | OPEN | P1 |
| 230 | input — text field plate | `packages/ui/src/components/base/inputs/InputFrame.vue:47` | `packages/ui/src/components/base/inputs/InputFrame.vue:47` h-9 rounded-xl px-3 gap-2; `:57` border-surface-5 bg-surface-4; `:4` focus-within ring-4 ring-brand-shadow; `:8` disabled opacity-50 | focus ring on focus-within, 4px | `crates/sift-ui/src/pages/settings.rs:1227-1228` 200x28; `:3825-3834` BUTTON_BG fill, no border | 8pt short, no border, no focus ring, no disabled state | OPEN | P0 |
| 231 | input — field width in rows | `apps/app-frontend/src/features/settings/instance-defaults/java-and-memory-settings/index.vue:118`<br>`apps/app-frontend/src/features/settings/instance-defaults/launch-hooks-settings.vue:96`<br>`apps/app-frontend/src/features/settings/system/resource-management-settings/index.vue:91` | `apps/app-frontend/src/features/settings/instance-defaults/java-and-memory-settings/index.vue:118` wrapper-class="w-full"; `apps/app-frontend/src/features/settings/instance-defaults/launch-hooks-settings.vue:96` w-full; `apps/app-frontend/src/features/settings/system/resource-management-settings/index.vue:91` w-full | static | `crates/sift-ui/src/pages/settings.rs:1227` FIELD_WIDTH 200 for every Text and Number | fields are 200px and right-aligned where the reference fills the 630px column | OPEN | P0 |
| 232 | input — number with unit | `apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:402-416` | `apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:402-416` type=number with a trailing "GiB" span, wrapper w-[42%] | static | `crates/sift-ui/src/pages/settings.rs:3810-3824` unit painted inside a 200pt plate | the unit is inside the field and the 42% column is ignored | OPEN | P1 |
| 233 | chips — chip shape and selected state | `packages/ui/src/components/base/Chips.vue:10`<br>`packages/ui/src/components/base/buttons/ButtonFrame.vue:31`<br>`packages/ui/src/components/base/Chips.vue:82`<br>`packages/ui/src/components/base/Chips.vue:78` | `packages/ui/src/components/base/Chips.vue:10` Button .btn md = h-9 rounded-xl px-2.5 text-base (`packages/ui/src/components/base/buttons/ButtonFrame.vue:31`); `packages/ui/src/components/base/Chips.vue:82` 1px transparent border; `packages/ui/src/components/base/Chips.vue:78` gap 0.5rem; selected = color brand, bg brand-highlight, border brand (`:97-101`) with a CheckIcon before the label (`:18-22`) | selected set on click, never emptying (`packages/ui/src/components/base/Chips.vue:65-72`) | `crates/sift-ui/src/pages/settings.rs:1221-1223` 26pt pill (radius 13), `:3794-3797` solid BRAND with ACCENT_CONTRAST text | 26px solid-green pills with no check mark where the reference is a 36px brand-outlined tint | OPEN | P0 |
| 234 | chips — chip placement on the Social tab | `packages/ui/src/layouts/wrapped/settings/AccountSocialSettings.vue:27-43` | `packages/ui/src/layouts/wrapped/settings/AccountSocialSettings.vue:27-43` heading, then Chips, then the description, left-aligned in a gap-2.5 column | static | `crates/sift-ui/src/pages/settings.rs:1681-1705` chips are the row's right-hand control | chips sit opposite the label instead of under it | OPEN | P0 |
| 235 | theme cards — grid | `packages/ui/src/layouts/shared/appearance-settings/components/appearance-settings-theme-selector.vue:111-112` | `packages/ui/src/layouts/shared/appearance-settings/components/appearance-settings-theme-selector.vue:111-112` repeat(auto-fit, minmax(12rem, 1fr)) with gap var(--gap-lg) = 16, so three ~199px cards across the 630px pane | static | `crates/sift-ui/src/pages/settings.rs:644-646` 96x64 cards with a 12pt gap, laid out at `:3528-3545` | cards are half the reference's width and leave the right half of the pane empty | OPEN | P0 |
| 236 | theme cards — card body | `packages/assets/styles/classes.scss:1139-1177` | `packages/assets/styles/classes.scss:1139-1177` width 100%, radius-md, 1px divider border, bg button-bg; preview p-1.5rem holding a rounded-lg example-card (border surface-4, bg surface-3, p-1rem, 32px icon and two bars) | static | `crates/sift-ui/src/pages/settings.rs:3548-3580` raised plate, no border, two bars, no example card | no divider ring and no miniature window illustration | OPEN | P1 |
| 237 | theme cards — label row | `packages/assets/styles/classes.scss:1179-1193`<br>`packages/ui/src/layouts/shared/appearance-settings/components/appearance-settings-theme-selector.vue:91-102` | `packages/assets/styles/classes.scss:1179-1193` padding var(--gap-md) var(--gap-lg), radio icon then the label at 16px; sun or moon marker from `packages/ui/src/layouts/shared/appearance-settings/components/appearance-settings-theme-selector.vue:91-102` | selected card: contrast text and brand radio (`packages/assets/styles/classes.scss:1151-1162`) | `crates/sift-ui/src/pages/settings.rs:648-656` 12pt label, 10pt dot, no markers | the label is 12px and neither marker is drawn | OPEN | P1 |
| 238 | language — language list rows | `packages/ui/src/components/base/buttons/CheckCircleButton.vue:31,37`<br>`packages/ui/src/layouts/wrapped/settings/language-settings/language-settings-selector.vue:275` | `packages/ui/src/components/base/buttons/CheckCircleButton.vue:31,37` size lg h-10, px-2, gap-4, rounded-[14px], 1px border; a 24px check circle at the end (`:47-53`); flag 24x16 (`packages/ui/src/layouts/wrapped/settings/language-settings/language-settings-selector.vue:275`), name 16 with the translated name at 14 (`:279-287`), coverage percent at the right (`:289-295`) | checked = border-brand bg-brand-highlight (`packages/ui/src/components/base/buttons/CheckCircleButton.vue:40`); hover bg-surface-3 (`:41`) | `crates/sift-ui/src/pages/settings.rs:1218` RADIO_ROW 26, `:3467-3521` 13pt label, 12pt dot, no flag, no row border | rows are 26pt with a dot where the reference is 40pt outlined rows with flags and a check circle | OPEN | P0 |
| 239 | language — no select in this tab | `packages/ui/src/layouts/wrapped/settings/language-settings/index.vue:24-31` | `packages/ui/src/layouts/wrapped/settings/language-settings/index.vue:24-31` a search field plus a radio group; no select control exists anywhere in the tab | static | `crates/sift-ui/src/pages/settings.rs:1525-1538` Control::Select carrying 5 of the languages as the category heading's control | a select the reference does not have, offering 5 of the ~30 locales | OPEN | P0 |
| 240 | language — warning callout | `packages/ui/src/components/base/Admonition.vue:4`<br>`language-settings/index.vue:7-9` | `packages/ui/src/components/base/Admonition.vue:4` rounded-2xl border p-4 grid; `:174-176` warning = border-brand-orange bg-bg-orange; placed mb-4 mt-2 (`language-settings/index.vue:7-9`) | static | `crates/sift-ui/src/pages/settings.rs:1519-1524` the warning is the heading's description line | a bordered orange callout is drawn as a paragraph | OPEN | P1 |
| 241 | java — per-version row control | `apps/app-frontend/src/features/settings/instance-defaults/java-and-memory-settings/java-installations.vue:32-40`<br>`apps/app-frontend/src/components/ui/JavaSelector.vue:5-19` | `apps/app-frontend/src/features/settings/instance-defaults/java-and-memory-settings/java-installations.vue:32-40` one block per version with its heading; `apps/app-frontend/src/components/ui/JavaSelector.vue:5-19` a full-width Input with a status button, then Install recommended, Detect and Browse (`:63-87`) | test status colours the status button green or red (`apps/app-frontend/src/components/ui/JavaSelector.vue:22-52`) | `crates/sift-ui/src/pages/settings.rs:1758-1794` one 200pt field per version, Detect and Browse drawn once below | per-row actions, the test status and Install recommended are missing | OPEN | P0 |
| 242 | java — memory block order | `apps/app-frontend/src/features/settings/instance-defaults/java-and-memory-settings/index.vue:76-106` | `apps/app-frontend/src/features/settings/instance-defaults/java-and-memory-settings/index.vue:76-106` heading, then the Slider at full width (min 512, step 64, snap points, min-label "512 MB", max-label "{n} GB"), then the description below it | static | `crates/sift-ui/src/pages/settings.rs:1795-1803` heading carries the description above a 160pt standalone slider | the description sits above the slider instead of below, and the min and max labels are absent | OPEN | P1 |
| 243 | resources — storage bar | `apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:295`<br>`apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:197-210` | `apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:295` flex h-3.5 gap-0.5 overflow-hidden rounded-full bg-surface-4; segments bg-blue, bg-purple and bg-gray with a white-to-black gradient (`apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:197-210`, `:300`) | segment flex-grow over 300ms (`apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:300`) | `crates/sift-ui/src/pages/settings.rs:295-296` 10pt bar radius 5, tints at `:3392-3425`, no gap | bar is 10pt not 14pt, segments are unpadded brand tints, and the category hues are wrong | OPEN | P0 |
| 244 | resources — storage legend | `apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:306-320` | `apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:306-320` grid 3 columns with gap-3 to gap-4, a size-3 dot, the label at 16px and a semibold tabular value | static | `crates/sift-ui/src/pages/settings.rs:3427-3450` one 12pt run, 8pt dots, no columns | the legend is an inline 12px line instead of a three-column list | OPEN | P1 |
| 245 | buttons — settings button | `packages/ui/src/components/base/buttons/ButtonFrame.vue:31`<br>`packages/ui/src/components/base/buttons/ButtonFrame.vue:45` | `packages/ui/src/components/base/buttons/ButtonFrame.vue:31` md h-9 rounded-xl px-2.5 text-base semibold; `packages/ui/src/components/base/buttons/ButtonFrame.vue:45` base bg-surface-4; `:135-140` inset 0 0 0 1px surface-5 plus 0 1px 1px | 150ms ease-out (`packages/ui/src/components/base/buttons/ButtonFrame.vue:18`), hover brightness var(--hover-brightness) (`:21`), active scale 0.97 (`:19`), focus ring-4 ring-brand-shadow (`:23`), disabled opacity-50 (`:24`) | `crates/sift-ui/src/pages/settings.rs:3902-3929` flat SUPER_RAISED_BG fill | no ring, no shadow, and no hover, press, focus or disabled state | OPEN | P1 |
| 246 | buttons — destructive button colour | `apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:344-355`<br>`apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:330-331` | `apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:344-355` Clear unused is type colored color red with a TrashIcon; Verify and repair is base, or orange while instances run (`apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:330-331`) | disabled while busy or when there is nothing to clear (`apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:332`, `:348`) | `crates/sift-ui/src/pages/settings.rs:1952-1955`, drawn as base plates at `:3902-3929` | Clear unused is not red and neither button shows its disabled state | OPEN | P1 |
| 247 | reset — reset-to-default affordance | `apps/app-frontend/src/features/settings/display/feature-flag-settings.vue:63-69`<br>`packages/ui/src/components/base/UnsavedChangesPopup.vue:90-92` | `apps/app-frontend/src/features/settings/display/feature-flag-settings.vue:63-69` a quiet Button on each flag row only, disabled at the default (`:65`); the global reset is the floating bar (`packages/ui/src/components/base/UnsavedChangesPopup.vue:90-92`) | disabled when the value equals its default | `crates/sift-ui/src/pages/settings.rs:3248-3274` a "Reset to default" text pill on every changed row | an invented control at the wrong scope and in the wrong shape | OPEN | P1 |
| 248 | tooltips — control tooltips | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:416-427`<br>`packages/ui/src/layouts/shared/appearance-settings/layout.vue:63-66`<br>`apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:325-345`<br>`apps/app-frontend/src/features/settings/display/feature-flag-settings.vue:64` | `apps/app-frontend/src/features/settings/app-settings-modal/index.vue:416-427` aria-label on the dev glyph; `packages/ui/src/layouts/shared/appearance-settings/layout.vue:63-66` tooltip on the disabled sync switch; `apps/app-frontend/src/features/settings/system/resource-management-settings/content-storage.vue:325-345` tooltips on both storage buttons; `apps/app-frontend/src/features/settings/display/feature-flag-settings.vue:64` a "Reset to default" tooltip | trigger: hover | `crates/sift-ui/src/pages/settings.rs:2188-2200` only standalone buttons get a tooltip | hover tooltips for row controls are missing | OPEN | P2 |
| 249 | evidence — capture at rest | — (the report gives no file:line) | not stated in source | UNKNOWN: .scratch/cap holds ref-home.png and ours-home*.png only, so there is no reference or ours capture of the Settings modal at 1440x900, and no image tool was available in this session | not applicable | every colour, pitch and shadow above is derived from source alone; a 1440x900 reference capture of each tab (dark and light) would settle what the render actually shows | OPEN | P0 |
| 250 | evidence — rail at a short window | — (the report gives no file:line) | not stated in source | UNKNOWN: at 900 tall the source's rail overflows (708pt wanted in a 553pt pane) and scrolls; whether the reference compresses at all below 900 is not stated in source | `crates/sift-ui/src/pages/settings.rs:2589-2608` compresses to a 30pt floor and calls it a deliberate deviation | an ours-versus-ref capture at 900 and at 700 tall would settle whether compression is ever acceptable | OPEN | P1 |

## 6. Skins

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 251 | skins — page frame | `apps/app-frontend/src/pages/Skins.vue:1102, 1357-1365` | route container `skin-layout box-border grow p-4` (16pt inset) laid out `minmax(0,1fr) minmax(0,2.5fr)` with `gap: 2.5rem`, collapsing to one column under 700px (apps/app-frontend/src/pages/Skins.vue:1102, 1357-1365) | static | crates/sift-ui/src/pages/skins.rs:117 (24pt inset), 697-709 (fixed 320pt preview plate + 16pt gap) | inset is 24 not 16, and the list column is a fixed 320pt plate rather than the 1fr of a 1fr/2.5fr split, so the grid starts ~40pt left of the reference's; no <700px single-column collapse | OPEN | P2 |
| 252 | skins — heading | `apps/app-frontend/src/pages/Skins.vue:1104-1106` | `h1.m-0.text-2xl.font-bold.flex.items-center.gap-2` carrying `appMessages.skinSelectorLabel` = "Skin selector" (apps/app-frontend/src/pages/Skins.vue:1104-1106) | static | crates/sift-ui/src/pages/skins.rs:366, 881-888 | matches: 24pt at bold, same string | DONE | — |
| 253 | skins — heading, stickiness | `apps/app-frontend/src/pages/Skins.vue:1103` | heading and preview sit in `sticky top-6 self-start p-2 pt-0` (apps/app-frontend/src/pages/Skins.vue:1103) so both stick while the list scrolls | sticky on scroll | crates/sift-ui/src/pages/skins.rs:881-888, 776-781 (page scroll area starts at the grid) | heading is painted once at the page top and never sticks; only the grid scrolls, so the reference's pinned preview column has no counterpart | OPEN | P2 |
| 254 | skins — 3D preview, box | `apps/app-frontend/src/pages/Skins.vue:1107-1109` | renderer wrapped in `ml-5 mt-4 flex h-[calc(80vh-1rem)] items-center justify-center`, `max-[700px]:h-[calc(50vh-1rem)]` (apps/app-frontend/src/pages/Skins.vue:1107-1109) | canvas fades `opacity 0 -> 100` over 500ms when ready (packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:56-57) | crates/sift-ui/src/pages/skins.rs:1400-1402, 383-387 (320pt wide, full page height) | no 80vh/50vh box and no 20pt `ml-5`; ours is a fixed-width plate whose height is whatever the page leaves | OPEN | P2 |
| 255 | skins — 3D preview, renderer | `packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:47-95, 93-94, 167-172, 328` | TresCanvas with `alpha`, antialias, `dpr [1,1.5]`, `NoToneMapping` at exposure 10, a 128-segment radial spotlight, `<TresAmbientLight :intensity="2">` and `<TresDirectionalLight :position="[-3,4,-2]" :intensity="1.2">`, plus a greeting-pose idle animation (packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:47-95, 93-94, 167-172, 328) | `Loading...` in `text-primary` centred until model+texture+fit resolve (packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:97-99, 330) | crates/sift-ui/src/pages/skins.rs:1396-1433 — a RAISED_BG plate with a 16x32 painted front view (or the Shirt glyph) | there is no 3D model, no lighting, no idle animation and no loading line: the page's defining element is a still plate (documented as `shortcut` at crates/sift-ui/src/pages/skins.rs:1396-1399) | OPEN | P0 |
| 256 | skins — 3D preview, camera | `packages/ui/src/composables/skin-rendering/use-skin-preview-fit.ts:24-29, 248-284` | framing presets: `page` fov 35, zoom 0.96, padding `{top .2, right .14, bottom .3, left .14}`; distance solved from the model's projected bounds against that padding (packages/ui/src/composables/skin-rendering/use-skin-preview-fit.ts:24-29, 248-284) | re-fits on container resize and on texture change (packages/ui/src/composables/skin-rendering/use-skin-preview-fit.ts:248-294) | absent — the view is a 2D `compose_front` elevation (crates/sift-ui/src/pages/skins.rs:1195, 1419-1423) | UNKNOWN as a camera comparison: there is no camera, so fov, padding and auto-fit cannot drift; the reference's framing rule has no home here until a renderer exists | OPEN | P0 |
| 257 | skins — 3D preview, initial rotation | `apps/app-frontend/src/pages/Skins.vue:1116`<br>`apps/app-frontend/src/components/ui/skin/EditSkinModal.vue:17`<br>`packages/ui/src/composables/skin-rendering/use-skin-preview-controls.ts:12` | `initial-rotation = Math.PI / 8` on the page (apps/app-frontend/src/pages/Skins.vue:1116) and on the modal (apps/app-frontend/src/components/ui/skin/EditSkinModal.vue:17), with the component's own default 15.75 and `+ Math.PI` applied on top (packages/ui/src/composables/skin-rendering/use-skin-preview-controls.ts:12) | static until dragged | absent — the front elevation has no yaw (crates/sift-ui/src/pages/skins.rs:1195-1220) | no initial angle at all; a port would need the 22.5 deg plus the half-turn offset to match the reference's first frame | OPEN | P1 |
| 258 | skins — 3D preview, drag to rotate | `packages/ui/src/composables/skin-rendering/use-skin-preview-controls.ts:17-39`<br>`packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:58-61` | pointerdown takes pointer capture, pointermove adds `deltaX * 0.01` radians, pointerup/pointerleave releases (packages/ui/src/composables/skin-rendering/use-skin-preview-controls.ts:17-39; packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:58-61) | container is `cursor-grab` (packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:5); a click with no drag plays an idle animation (packages/ui/src/composables/skin-rendering/use-skin-preview-controls.ts:41-47) | absent — no pointer handling anywhere in skins.rs except the model tabs (crates/sift-ui/src/pages/skins.rs:996-1003) | no drag, no grab cursor, no click reaction | OPEN | P1 |
| 259 | skins — rotation control | `packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:13-18`<br>`packages/ui/src/composables/skin-rendering/use-skin-preview-fit.ts:40, 332-346` | the only rotation affordance is a centred hint: `UnfoldHorizontalIcon size-5` + "Drag to rotate" in `text-base font-medium leading-6 text-primary` (packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:13-18), placed 64pt under the model's feet (packages/ui/src/composables/skin-rendering/use-skin-preview-fit.ts:40, 332-346) | static (no hover change); `bottom: calc(15% + 64px)` when fit is unavailable (packages/ui/src/composables/skin-rendering/use-skin-preview-fit.ts:339) | absent | there is no button, slider or reset for rotation in 0.22.0 — only this hint — and ours has neither, so a future drag would be undiscoverable | OPEN | P1 |
| 260 | skins — preview nametag | `apps/app-frontend/src/pages/Skins.vue:348, 1115`<br>`packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:30-45, 360-364, 391-399` | the account's username over the model in `font-minecraft` on a two-gradient `nametag-bg`, `translate(-50%, calc(-100% - .75rem))`, hidden by the `hideNametagSkinsPage` setting (apps/app-frontend/src/pages/Skins.vue:348, 1115; packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:30-45, 360-364, 391-399) | static | absent | no nametag and no setting hook ⇒ blocked on the account service — the nametag is the signed-in username | BLOCKED-SERVICE | P3 |
| 261 | skins — preview badge | `Skins.vue:1120-1127, 174-177`<br>`packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:36-41` | while `hasPendingSkinChange`: an `EyeIcon size-5` pill, `rounded-full border-brand-blue bg-bg-blue px-3 py-1 text-base font-semibold leading-6 text-brand-blue`, set `bottom-[calc(100%+1rem)]` above the nametag (Skins.vue:1120-1127, 174-177; packages/ui/src/components/skin/SkinPreviewRendererImpl.vue:36-41) | appears and disappears with the pending change | absent — `State.selected` is written once and never differs from the applied skin (crates/sift-ui/src/pages/skins.rs:484, 511) | the page has no applied-vs-previewed distinction, so the badge could not be shown even if drawn | OPEN | P1 |
| 262 | skins — Apply button | `apps/app-frontend/src/pages/Skins.vue:1156-1177, 178-185, 771-800` | `type="colored" color="brand" size="lg"` + CheckIcon, SpinnerIcon while applying, disabled when signed out / applying / read-only, tooltip "Sign in to apply skins." when signed out (apps/app-frontend/src/pages/Skins.vue:1156-1177, 178-185, 771-800) | spinner spin while `isApplyingSkin`; a 429 opens a "Slow down!" notification (apps/app-frontend/src/pages/Skins.vue:141-149, 788-793) | absent — no Apply control exists in skins.rs | the page's primary action is missing entirely ⇒ blocked on the auth/account service — Apply posts the skin, and its tooltip is "Sign in to apply skins." | BLOCKED-SERVICE | P0 |
| 263 | skins — Reset button | `apps/app-frontend/src/pages/Skins.vue:1138-1155` | `type="base" size="lg"` with `RotateCounterClockwiseIcon` and the reset label, shown beside Apply only while a change is pending (apps/app-frontend/src/pages/Skins.vue:1138-1155) | static | absent | a previewed skin cannot be reverted | OPEN | P1 |
| 264 | skins — Edit button | `apps/app-frontend/src/pages/Skins.vue:1179-1188, 186-189`<br>`apps/app-frontend/src/components/ui/skin/EditSkinModal.vue:11-125` | `EditIcon` + "Edit skin", `type="base" size="lg"`, shown *instead of* Apply/Reset when nothing is pending (apps/app-frontend/src/pages/Skins.vue:1179-1188, 186-189); opens EditSkinModal with the texture, arm style, cape and Save (apps/app-frontend/src/components/ui/skin/EditSkinModal.vue:11-125) | press opens the modal | crates/sift-ui/src/pages/skins.rs:376, 1449-1462 — a painted 120x32 plate with the label | painted only: no press target, no modal, no arm-style/cape editor | OPEN | P1 |
| 265 | skins — Ears feature notice | `apps/app-frontend/src/pages/Skins.vue:1190-1251, 190-201` | a 40pt rounded `ears-mod.png`, the copy "This skin uses features from the {ears} mod" with an Ears link, a toggle Button and a small `Toggle` (apps/app-frontend/src/pages/Skins.vue:1190-1251, 190-201) | container query under 300px swaps the button for a switch (apps/app-frontend/src/pages/Skins.vue:1320-1354) | absent | ears-mod detection and its toggle are not modelled (acceptable while no texture is parsed) | OPEN | P3 |
| 266 | skins — demo account bar | `Skins.vue:1281-1308, 202-213` | signed-out foot bar: `sticky bottom-0 p-4 pt-0`, inner `max-w-5xl rounded-[20px] border-surface-5 bg-surface-3 p-4`, `InfoIcon size-6 text-blue`, title `text-lg font-semibold text-contrast`, copy `text-base text-primary`, brand "Sign in to Microsoft" with WindowsIcon (Skins.vue:1281-1308, 202-213) | hidden once an account exists; spinner while logging in (apps/app-frontend/src/pages/Skins.vue:1303) | crates/sift-ui/src/pages/skins.rs:1405-1410, 1464-1490 — a 72pt bar *inside* the preview plate, three stacked items, no icon | wrong container (inside the column, not a page-foot bar), wrong width (unconstrained vs `max-w-5xl` centring), no InfoIcon, 72pt fixed height | OPEN | P2 |
| 267 | skins — selector, section header | `apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:373-404, 52` | accordion header at 28pt: `DropdownIcon size-6` that rotates 180 in 300ms, title `text-xl font-semibold leading-7 text-primary`, `gap-[6px]`, plus an info `UnknownIcon size-5` when the section has a tooltip (apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:373-404, 52) | chevron rotates 300ms; the section collapses and its grid height is recomputed (apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:314-342) | crates/sift-ui/src/pages/skins.rs:1088-1109 (static ChevronDown, 20pt title, 6pt gap) | header is not clickable or collapsible, the chevron does not animate, the info tooltip is missing | OPEN | P2 |
| 268 | skins — selector, column count | `apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:114-128`<br>`apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:409, 532` | fixed counts by window width: 3 default, 4 at >=1300, 5 at >=1750, 6 at >=2050 (apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:114-128), applied as `grid-cols-3 min-[1300px]:grid-cols-4 ...` (apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:409, 532) | relayout on resize | crates/sift-ui/src/pages/skins.rs:1126-1143, 1154-1163 — `floor((gridWidth + 12) / (minWidth + 12))` with a 160/128pt minimum | different rule: ours counts columns from the column's width, so 1440 gives five 180pt tiles where the reference's ~660pt list column gives four ~156pt ones (crates/sift-ui/src/pages/skins.rs:65-79 states the trade) | OPEN | P1 |
| 269 | skins — selector, grid gap and tile | `apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:409`<br>`apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:428, 449` | grid `gap-3` = 12 (apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:409), tile `aspect-[31/40] w-full rounded-[20px]` (apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:428, 449) | static | crates/sift-ui/src/pages/skins.rs:237 (12), 251-262, 1167-1169 | matches | DONE | — |
| 270 | skins — Add-skin tile | `apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:426-443`<br>`packages/ui/src/components/skin/SkinLikeTextButton.vue:58-95` | first tile of the Saved grid: `border border-dashed border-surface-5 bg-surface-2 rounded-[20px]`, `PlusIcon size-8`, "Add skin" at `text-base font-semibold leading-6`, "Drag and drop" at `text-sm font-medium leading-5 text-primary`, `gap-4` between glyph and text, `gap-0.5` between lines (apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:426-443; packages/ui/src/components/skin/SkinLikeTextButton.vue:58-95) | drag-active swaps the edge to `border-brand bg-brand-highlight` and the text to `text-brand` (packages/ui/src/components/skin/SkinLikeTextButton.vue:27, 59-66, 83) | crates/sift-ui/src/pages/skins.rs:1622-1664 (tile), 814 (drawn at slot 0) | tile matches at rest; no drag-active state and no hover (`hover:bg-surface-3`) because nothing on the card is interactive | OPEN | P2 |
| 271 | skins — Upload affordance | `apps/app-frontend/src/pages/Skins.vue:1087-1093, 843-847`<br>`apps/app-frontend/src/pages/Skins.vue:893-941, 975-988`<br>`UI/components/skin/SkinLikeTextButton.vue:23-70`<br>`APP/components/ui/skin/VirtualSkinSectionList.vue:434-436`<br>`APP/pages/Skins.vue:1274`<br>`af/pages/Skins.vue:866-878` | a hidden `<input type=file accept=image/png>` opened by the add tile (apps/app-frontend/src/pages/Skins.vue:1087-1093, 843-847), plus native OS drag-drop onto that tile via `onDragDropEvent` (apps/app-frontend/src/pages/Skins.vue:893-941, 975-988) ‖ interaction.md row 50 — element "drag & drop — drop a skin file on the add tile": `UI/components/skin/SkinLikeTextButton.vue:23-70` (`dragenter/dragover/dragleave/drop`), wired `APP/components/ui/skin/VirtualSkinSectionList.vue:434-436`, `APP/pages/Skins.vue:1274` ‖ file-management.md row 6 — element "Drag & drop — Skins: the one per-type drop rule": Only `.png` (`af/pages/Skins.vue:866-878`); the drop must land inside the Add-skin button's rect (`:880-891`) or it is silently ignored (`:914-929`); button gets a drag-active flag (`:908-912`, `:943-957`); failure raises `droppedFileErrorTitle` (`:935-940`). | the tile highlights while a PNG is dragged over it (apps/app-frontend/src/pages/Skins.vue:880-891, 943-957) ‖ interaction.md row 50: — (this report has no states column) ‖ file-management.md row 6: — (this report has no states column) | crates/sift-ui/src/pages/skins.rs:1032-1061 — a brand "Add skin" button in the toolbar row, painted but not wired ‖ interaction.md row 50: the tile is drawn (`SRC/pages/skins.rs:1622`) and is not a drop target; `"Drag and drop"` is printed at `:406-409` ‖ file-management.md row 6: The Skins page is drawn from static data (`sift-ui/pages/skins.rs`), no drop path (same `main.rs:79-91` frame loop). | the reference has no toolbar Upload at all; ours invents one, wires neither it nor the tile, and has no file input or drop target ‖ interaction.md row 50: the tile advertises a gesture it cannot accept ‖ file-management.md row 6: Dropping a skin PNG does nothing. | OPEN | P1 |
| 272 | skins — card, hover and focus | `packages/ui/src/components/skin/SkinButton.vue:55, 173-187` | `:hover, :focus-within` set `border-color: var(--surface-5)`, `background: var(--surface-4)` and a two-layer shadow, over `transition-[border-color,box-shadow] duration-200`; focus-visible adds a 2pt brand outline at 2pt offset (packages/ui/src/components/skin/SkinButton.vue:55, 173-187) | 200ms on border-color and shadow | crates/sift-ui/src/pages/skins.rs:1499-1512 — static `SUPER_RAISED_BG` plus a `SURFACE_5` stroke; no interact call | no hover, no focus ring and no press: the tile is paint only; the resting border is also `--surface-5` where the reference's is `--surface-4` (packages/ui/src/components/skin/SkinButton.vue:155) ⇒ renderer risk: multi-layer shadow — two stacked layers on every tile hover | RENDERER-RISK | P1 |
| 273 | skins — card, overlay actions | `packages/ui/src/components/skin/SkinButton.vue:118-123`<br>`apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:461-482, 503-524` | an `absolute inset-x-0 bottom-3 px-3` row revealed from `translate-y-2 opacity-0` to `0 / 100` over `transition-all duration-200` on `group-hover` or `group-focus-within`: a brand `Button` with EditIcon (aria-label "Edit skin") and a red `IconButton` with TrashIcon `!rounded-[100%]`, the latter `v-show="!skin.is_equipped"` (packages/ui/src/components/skin/SkinButton.vue:118-123; apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:461-482, 503-524) | 8pt slide plus fade, 200ms, on hover and focus-within; Delete hidden while equipped | absent — the card draws name, author and model tag instead (crates/sift-ui/src/pages/skins.rs:1499-1601) | no per-card Edit or Delete, and no reveal animation; deleting is impossible from the grid | OPEN | P0 |
| 274 | skins — card, selected state | `packages/ui/src/components/skin/SkinButton.vue:90-96, 189-197` | a 24pt disc at `right-3 top-3`: `bg-contrast` fill plus `CheckIcon size-4 invert [stroke-width:3]`; the tile itself gets `border-color: color-mix(--color-text-primary 40%)`, `filter: brightness(1.1)` (packages/ui/src/components/skin/SkinButton.vue:90-96, 189-197) | applied while `selected`; `aria-pressed` on the inner button (packages/ui/src/components/skin/SkinButton.vue:80) | absent on the saved grid; only the first default tile carries a brand check (crates/sift-ui/src/pages/skins.rs:848-858) | the applied skin is not marked on the tiles at all (the reference marks the *selected* one, which may differ) | OPEN | P1 |
| 275 | skins — card, pending dot | `packages/ui/src/components/skin/SkinButton.vue:85-88` | a 12pt `bg-green` dot at `right-3 top-3` when `active && !selected` (packages/ui/src/components/skin/SkinButton.vue:85-88) | static | absent | no pending state to mark | OPEN | P2 |
| 276 | skins — card, preview skeleton | `packages/ui/src/components/skin/SkinButton.vue:98-100, 128-151` | until the baked image loads: `.skeleton-loader` at `aspect-ratio: 31/40` holding `.skeleton`, `linear-gradient(145deg, surface-2, surface-3)` animating `skeleton-pulse 2700ms ease-in-out infinite` on `brightness(.95 -> 1.15)`, disabled under `prefers-reduced-motion` (packages/ui/src/components/skin/SkinButton.vue:98-100, 128-151) | 2700ms infinite pulse, reduced-motion off | absent — textures are composed on first draw and painted immediately (crates/sift-ui/src/pages/skins.rs:1540-1550) | no per-card loading state ⇒ renderer risk: animated gradient — 2700ms skeleton-pulse over a 145deg linear-gradient | RENDERER-RISK | P2 |
| 277 | skins — Delete confirmation | `apps/app-frontend/src/pages/Skins.vue:1094-1100, 166-173, 369-390` | ConfirmModal titled "Are you sure you want to delete this skin?"/copy "This will permanently delete the selected skin. This action cannot be undone.", proceed label Delete (apps/app-frontend/src/pages/Skins.vue:1094-1100, 166-173, 369-390) | modal show/hide | absent | no delete flow and therefore no confirmation | OPEN | P1 |
| 278 | skins — saved-skin reorder | `apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:406-424, 565-577`<br>`apps/app-frontend/src/pages/Skins.vue:655-701`<br>`APP/components/ui/skin/VirtualSkinSectionList.vue:16, 159-161, 227, 289-311, 408` | `vuedraggable` over the saved grid with `animation 250`, `swap-threshold 1`, `force-fallback`, ghosts at `opacity .35` and `.9 / pointer-events: none`, persisted through `set_custom_skin_order` with rollback on failure (apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:406-424, 565-577; apps/app-frontend/src/pages/Skins.vue:655-701) ‖ interaction.md row 49 — element "drag & drop — reorder saved skins": `APP/components/ui/skin/VirtualSkinSectionList.vue:16, 159-161, 227, 289-311, 408` | 250ms reorder easing, drag ghost, grabbing cursor ‖ interaction.md row 49: — (this report has no states column) | absent ‖ interaction.md row 49: none; tiles painted `SRC/pages/skins.rs:779-820` | saved tiles cannot be reordered and no order is stored ‖ interaction.md row 49: absent | OPEN | P2 |
| 279 | skins — Default skins | `apps/app-frontend/src/pages/Skins.vue:290-317, 73`<br>`apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:530-559` | a stack of accordion *sections* — "Default skins", "MINECON Earth 2017", "Builders & Biomes", … , "Modrinth Pride" with an info tooltip and a campaign gate — each a 3-6 column grid of BakedSkinButtons with select and an Edit overlay but no Delete (apps/app-frontend/src/pages/Skins.vue:290-317, 73; apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:530-559) | each section collapses independently; Pride appears only with the badge (apps/app-frontend/src/pages/Skins.vue:287-294) | crates/sift-ui/src/pages/skins.rs:818-859 — one painted "Default skins" line and a single row of 8 art tiles at 0.66x card width, first one check-badged | one invented row instead of the reference's collapsible, titled sections; no per-section titles, tooltip, selection or Edit overlay, and the row is not the 31/40 card shape | OPEN | P1 |
| 280 | skins — empty state | `apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:426-443` | none: with nothing saved the Saved-skins section still renders only its add tile, no prose anywhere on the page (apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:426-443) | static | crates/sift-ui/src/pages/skins.rs:600-654, 744-757 — NoSkins draws the tile alone (matches), NoMatches draws a launcher-only magnifier plus "Try a different search or filter." (crates/sift-ui/src/pages/skins.rs:425, 1721-1741) | the empty state matches the reference; NoMatches is an addition the reference has no equivalent for (the module says so at crates/sift-ui/src/pages/skins.rs:642-645) | OPEN | P3 |
| 281 | skins — account gating | `apps/app-frontend/src/pages/Skins.vue:349-353, 844, 991`<br>`packages/ui/src/components/skin/SkinButton.vue:199-205` | `isSkinManagementReadOnly` (offline or auth server unreachable) short-circuits every mutator, and `skin-button--disabled` dims to `opacity: .65` with `cursor: not-allowed` (apps/app-frontend/src/pages/Skins.vue:349-353, 844, 991; packages/ui/src/components/skin/SkinButton.vue:199-205) | disabled look, no hover | absent | no read-only concept; only the model tabs accept input ⇒ blocked on the auth service — the read-only gate is "offline or auth server unreachable" | BLOCKED-SERVICE | P3 |

## 7. Screenshots

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 282 | screenshots — page frame | `apps/app-frontend/src/pages/Screenshots.vue:26-28` | `box-border h-full p-6` around the page, with `show-heading` (apps/app-frontend/src/pages/Screenshots.vue:26-28) | static | crates/sift-ui/src/pages/screenshots.rs:111, 715-728 | matches (24pt inset, heading on) | DONE | — |
| 283 | screenshots — page root | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1485-1489` | `flex h-full w-full flex-col gap-3`, plus `justify-center` while `screenshots.length === 0` (apps/app-frontend/src/components/ui/screenshots-page/index.vue:1485-1489) | static | crates/sift-ui/src/pages/screenshots.rs:115, 746-756 (empty state centred in the page rect) | matches | DONE | — |
| 284 | screenshots — heading | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1491-1493`<br>`apps/app-frontend/src/pages/Screenshots.vue:13` | `m-0 text-2xl font-bold text-contrast` = "Screenshots", rendered only when the list is non-empty (apps/app-frontend/src/components/ui/screenshots-page/index.vue:1491-1493; apps/app-frontend/src/pages/Screenshots.vue:13) | hidden in the empty state | crates/sift-ui/src/pages/screenshots.rs:345, 845-865 | type matches; ours adds a total count pill beside the heading, an element the reference only shows per group (apps/app-frontend/src/components/ui/screenshots-page/section.vue:148-150) | DONE | P3 |
| 285 | screenshots — toolbar, row 1 | `apps/app-frontend/src/components/ui/screenshots-page/toolbar.vue:36-49` | `Input` with SearchIcon, placeholder "Search", `clearable`, `wrapper-class min-w-[16rem] flex-1`; then `Button` with `SquarePlusIcon` + "New group" (apps/app-frontend/src/components/ui/screenshots-page/toolbar.vue:36-49) | clear button appears when the field has text | crates/sift-ui/src/pages/screenshots.rs:146, 867-885, 920-983 | the plate is painted, not typable, and has no clear affordance; New group is inert RE-VERIFIED 2026-11-01: the plate takes the keyboard, draws the query and its X clears (screenshots.rs:1499-1562), and New group answers with a toast that says groups are not built (:1189-1191): the press lands, the feature behind it does not exist. | OPEN-WITH-CAUSE | P1 |
| 286 | screenshots — toolbar, row 2 | `apps/app-frontend/src/components/ui/screenshots-page/toolbar.vue:50-85`<br>`apps/app-frontend/src/components/ui/screenshots-page/index.vue:247-253` | sort `Combobox` with `ArrowUpDownIcon` prefix (options Newest / Oldest / Name) and group `Combobox` with `LayoutGridIcon` prefix (options Custom group / Instance / Date / No grouping), both `w-max`, `dropdown-min-width 160px` (apps/app-frontend/src/components/ui/screenshots-page/toolbar.vue:50-85; apps/app-frontend/src/components/ui/screenshots-page/index.vue:247-253) | hover, focus and an open dropdown; selected label only | crates/sift-ui/src/pages/screenshots.rs:887-903, 1042-1105 — two painted selects at the stored values | no dropdown, no hover, no press; group-by is pinned to "Date" with no way to pick another mode RE-VERIFIED 2026-11-01: both controls open a list and take a choice — sort at screenshots.rs:1322, group at :1341, through `open_list` (:1857) over the `Sort` and `GroupBy` enums (:488-542). | VERIFIED | P1 |
| 287 | screenshots — toolbar defaults and persistence | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:149-150` | `sort` defaults to `newest`, `groupBy` to `date`, both kept in storage keys (`screenshots-sort-*`, `screenshots-group-v2-*`) (apps/app-frontend/src/components/ui/screenshots-page/index.vue:149-150) | restored on next visit | crates/sift-ui/src/pages/screenshots.rs:419-450, 455 (Sort::default = Newest), 361 ("Date") | values match; nothing is persisted between sessions | OPEN | P3 |
| 288 | screenshots — grouping by month | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:985-1009, 227` | `getDateGroup` buckets Today, Yesterday, This week, This month, then `date:month:YYYY-MM` titled with `useFormatDateTime({ month: 'long', year: 'numeric' })` i.e. "October 2026" (apps/app-frontend/src/components/ui/screenshots-page/index.vue:985-1009, 227) | the first four buckets depend on the reader's clock; groups re-sort by their newest member (apps/app-frontend/src/components/ui/screenshots-page/index.vue:516-524) | crates/sift-ui/src/pages/screenshots.rs:54-63, 577-631 (month bucket only, "October 2026") | the four relative headings are missing and the module records why (crates/sift-ui/src/pages/screenshots.rs:86-87); month buckets and titles match | DONE | P1 |
| 289 | screenshots — group header | `apps/app-frontend/src/components/ui/screenshots-page/section.vue:112-154` | an `h-10` row, `border-b border-surface-5`, `DropdownIcon size-5` rotating `rotate-180` in `duration-300`, title `text-base font-semibold text-primary`, a `TagItem` count in `border-surface-3 bg-surface-2`, `gap-2` (apps/app-frontend/src/components/ui/screenshots-page/section.vue:112-154) | chevron rotates 300ms; header hover lifts title to `text-contrast` | crates/sift-ui/src/pages/screenshots.rs:1147-1191 (40pt row, rule, ChevronUp, 16pt semibold title, count pill) | 40pt row, rule and title type match; the chevron is static and the hover colour change is missing | DONE | P2 |
| 290 | screenshots — group collapse | `apps/app-frontend/src/components/ui/screenshots-page/section.vue:155-167`<br>`apps/app-frontend/src/components/ui/screenshots-page/group.vue:99-120, 208` | an Accordion whose `open-by-default` is `hideHeader or !collapsed`, `force-open` while searching, and whose grid unmounts 300ms after collapsing (apps/app-frontend/src/components/ui/screenshots-page/section.vue:155-167; apps/app-frontend/src/components/ui/screenshots-page/group.vue:99-120, 208) | 300ms delayed unmount after collapse; `mt-2.5` grid offset (apps/app-frontend/src/components/ui/screenshots-page/section.vue:164) | absent — every group is drawn open and unclickable (crates/sift-ui/src/pages/screenshots.rs:1162, 1195) | groups cannot be collapsed RE-VERIFIED 2026-11-01: a header's press folds its group and the block height follows it (screenshots.rs:2473-2481, height :1070-1081, test :3633). | VERIFIED | P2 |
| 291 | screenshots — group header actions | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1592-1615` | for a custom group only: rename and delete `IconButton`s revealed at `opacity-0 -> 100` over `duration-250` on `group-hover/header` and `focus-within` (apps/app-frontend/src/components/ui/screenshots-page/index.vue:1592-1615) | fade 250ms on header hover | absent | custom groups are not modelled, so the header carries no actions | OPEN | P3 |
| 292 | screenshots — grid, columns | `apps/app-frontend/src/components/ui/screenshots-page/group.vue:210`<br>`apps/app-frontend/src/components/ui/screenshots-page/index.vue:527-531` | `grid-cols-1 gap-3 sm:grid-cols-2 2xl:grid-cols-4` → 1 column, 2 at 640, 4 at 1536 (apps/app-frontend/src/components/ui/screenshots-page/group.vue:210), the same thresholds used for virtualization (apps/app-frontend/src/components/ui/screenshots-page/index.vue:527-531) | relayout on resize | crates/sift-ui/src/pages/screenshots.rs:241-254, 666-677 | matches, including both breakpoints and the 4-column branch at 1536 | DONE | — |
| 293 | screenshots — grid, gap and card aspect | `apps/app-frontend/src/components/ui/screenshots-page/group.vue:210`<br>`apps/app-frontend/src/components/ui/screenshots-page/card.vue:172`<br>`apps/app-frontend/src/components/ui/screenshots-page/index.vue:542-566` | `gap-3` = 12 (apps/app-frontend/src/components/ui/screenshots-page/group.vue:210); card `aspect-video` = 16:9 (apps/app-frontend/src/components/ui/screenshots-page/card.vue:172); a group's block is header 40 + content 10 + grid + spacing 12 (apps/app-frontend/src/components/ui/screenshots-page/index.vue:542-566) | static | crates/sift-ui/src/pages/screenshots.rs:217, 221-233, 258 (9/16), 699-707 | matches: 12pt gap, 16:9 cards, 40/10/12 block arithmetic | DONE | — |
| 294 | screenshots — card, box | `apps/app-frontend/src/components/ui/screenshots-page/card.vue:172` | `aspect-video min-w-0 cursor-pointer overflow-hidden rounded-xl border border-surface-5 bg-surface-2 shadow-sm` (apps/app-frontend/src/components/ui/screenshots-page/card.vue:172) | hover `brightness-110`; selected `!border-contrast brightness-110`; drag `opacity-50`; highlighted `ring-2 ring-brand animate-pulse` (apps/app-frontend/src/components/ui/screenshots-page/card.vue:173-178) | crates/sift-ui/src/pages/screenshots.rs:262, 1215-1249 | box, radius and border match; hover brightness, selected border and drag dimming are absent | OPEN | P2 |
| 295 | screenshots — card, image | `apps/app-frontend/src/components/ui/screenshots-page/card.vue:37-41, 215-231, 281-285` | 512pt thumbnail via `useImageThumbnail(path, 512, modified_at)`, `<img loading=eager decoding=async object-cover>` fading in over `350ms ease-in-out`, over a `bg-button-bg animate-pulse` placeholder until it decodes (apps/app-frontend/src/components/ui/screenshots-page/card.vue:37-41, 215-231, 281-285) | 350ms fade; pulse until decoded | crates/sift-ui/src/pages/screenshots.rs:1220-1239 — a `SUPER_RAISED_BG` plate, the image call site commented out | no thumbnails at all (the record carries no path), so the grid is a wall of empty plates | DONE | P0 |
| 296 | screenshots — select mode, checkbox | `apps/app-frontend/src/components/ui/screenshots-page/card.vue:194-214` | a `size-[50px]` hit box at `right-0.5 top-0` holding a 24pt circle: `border-2 border-primary bg-transparent` at `opacity-0`, revealed on `group-hover` and `group-focus-within` over `duration-200 ease-out`; when selected it drops the border, fills `bg-contrast` and draws `CheckIcon size-4 invert [stroke-width:3]`, always visible (apps/app-frontend/src/components/ui/screenshots-page/card.vue:194-214) | 200ms opacity on hover/focus; `aria-pressed` on both the card and the button (apps/app-frontend/src/components/ui/screenshots-page/card.vue:189, 202) | absent | no selection affordance and no hover reveal RE-VERIFIED 2026-11-01: the checkbox is a 50-point hit target (screenshots.rs:2668-2678) revealed on hover and always drawn once selected (:2713-2738). | VERIFIED | P0 |
| 297 | screenshots — select mode, activation | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1022-1046` | `activateScreenshot` toggles one card when a selection is already active or when shift is held, otherwise it opens the viewer; `toggleScreenshotSelection` is a plain set toggle gated on `bulkBusy` (apps/app-frontend/src/components/ui/screenshots-page/index.vue:1022-1046) | click, shift-click, Enter/Space on a focused card (apps/app-frontend/src/components/ui/screenshots-page/card.vue:76-84) | absent | there is no selection mode, no click-to-select and no viewer, so a card press does nothing RE-VERIFIED 2026-11-01: a press toggles the selection when one is active or Shift is held and otherwise opens the viewer (screenshots.rs:2590-2601; the viewer is :2328). | VERIFIED | P0 |
| 298 | screenshots — select mode, shift-range | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1035` | not stated in source: shift-click toggles a single screenshot, there is no anchor, range or `shiftKey` range code anywhere in the component (apps/app-frontend/src/components/ui/screenshots-page/index.vue:1035) | single toggle only | absent | UNKNOWN as a range feature — 0.22.0 has none, so nothing should be ported for it | NOT-IN-SOURCE | — |
| 299 | screenshots — card, caption overlay | `apps/app-frontend/src/components/ui/screenshots-page/card.vue:232-277`<br>`apps/app-frontend/src/components/ui/screenshots-page/card.vue:245` | `absolute inset-x-0 bottom-0 bg-gradient-to-t from-surface-1 to-transparent p-3 pt-[120px]`, file name `truncate text-sm font-semibold`, second line `truncate text-xs text-secondary`, then edit / copy / more `IconButton`s with `bg-surface-2 text-contrast hover:bg-surface-3` (apps/app-frontend/src/components/ui/screenshots-page/card.vue:232-277) ‖ motion.md row 64 — element "cards — screenshot card caption": `apps/app-frontend/src/components/ui/screenshots-page/card.vue:245` `translate-y-1 ... opacity-0 transition group-hover:translate-y-0 group-hover:opacity-100`; `:172` `transition-[filter] hover:brightness-110` | whole overlay `opacity-0 -> 100` over `duration-200` on `group-hover` and `group-focus-within`; actions slide `translate-y-1 -> 0` ‖ motion.md row 64: 4-point rise plus fade on hover; duration not stated in source | crates/sift-ui/src/pages/screenshots.rs:1265-1351 — always drawn: 3-band scrim, file name, "{instance} · {date}", three 36pt action discs ‖ motion.md row 64: `crates/sift-ui/src/pages/screenshots.rs:1255-1256` (caption drawn at rest) | the overlay is drawn at rest instead of on hover; the second line is the instance-and-date string where the card shows instance name OR time (apps/app-frontend/src/components/ui/screenshots-page/card.vue:240); the actions are inert ‖ motion.md row 64: no hover state RE-VERIFIED 2026-11-01: the overlay is drawn only while the card is hovered (screenshots.rs:2709-2711) and its three buttons are live — Open, Delete, More (:2613-2644, rects :2760-2768). | VERIFIED | P1 |
| 300 | screenshots — context menu | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1058-1098, 1432-1443`<br>`apps/app-frontend/src/components/ui/screenshots-page/card.vue:191, 267-275` | right-click opens `ContextMenu` at the pointer: Edit and Copy (contextmenu only), Show in folder, Go to instance, a divider, then Delete in `tone: 'red'` (apps/app-frontend/src/components/ui/screenshots-page/index.vue:1058-1098, 1432-1443; apps/app-frontend/src/components/ui/screenshots-page/card.vue:191, 267-275) | kebab and right-click both open it; menu closes on activate | absent — no menu and no contextmenu handling | no per-card menu: edit, copy, show in folder, go to instance and delete are unreachable RE-VERIFIED 2026-11-01: a secondary click opens the menu at the pointer (screenshots.rs:2603-2609) carrying Open, Show in folder and Delete (MENU_ITEMS :561, `card_menu` :1910-1963). | VERIFIED | P1 |
| 301 | screenshots — delete | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:278-284, 1196-1209, 1404-1415`<br>`index.vue:1058-1098` | ConfirmModal "Delete screenshot" / "Permanently delete {name}? This action cannot be undone." with a red proceed, then a success toast "Screenshot deleted" (apps/app-frontend/src/components/ui/screenshots-page/index.vue:278-284, 1196-1209, 1404-1415) ‖ file-management.md row 46 — element "Screenshots — Delete one, with confirmation": Options menu → Delete opens a modal (`index.vue:1058-1098`, `:1052-1056`), confirmed with `delete_screenshots([key])`, success toast `Screenshot deleted` (`:1196-1209`; copy at `:278-283`); deleting from the viewer closes it (`:1201`). | modal show, mutation pending, toast ‖ file-management.md row 46: — (this report has no states column) | absent ‖ file-management.md row 46: None — the page has no press, hover or menu at all (`sift-ui/pages/screenshots.rs:77-93`, `:715-724`). | no delete path at all, so nothing can be removed from the folder ‖ file-management.md row 46: No destructive action, which is why nothing can be lost yet. RE-VERIFIED 2026-11-01: deletion is two-step — the `x` arms, the strip confirms (screenshots.rs:2576-2586) — and `delete_paths` removes the files (:2246). | VERIFIED | P0 |
| 302 | screenshots — bulk delete | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:307-319, 1211-1220, 1416-1423`<br>`index.vue:1701` | "Delete selected screenshots" / "Delete {count, plural, one {# screenshot} other {# screenshots}}? This action cannot be undone." / "{count, plural, ...} deleted" (apps/app-frontend/src/components/ui/screenshots-page/index.vue:307-319, 1211-1220, 1416-1423) ‖ file-management.md row 47 — element "Screenshots — Delete a selection": Floating bar → modal → `delete_screenshots(keys)` with `{count}` toast (`index.vue:1701`, `:308-317`, `:1211-1224`). | disabled while `bulkBusy` ‖ file-management.md row 47: — (this report has no states column) | absent ‖ file-management.md row 47: None. | blocked by the missing selection mode as well as the missing flow ‖ file-management.md row 47: No selection, no bulk delete. RE-VERIFIED 2026-11-01: the selection bar's Delete calls `delete_paths` with every selected path (screenshots.rs:2236), so the selection mode this row was blocked on exists (:2590-2601). | VERIFIED | P1 |
| 303 | screenshots — selection toolbar | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1645-1707`<br>`packages/ui/src/components/base/FloatingActionBar.vue:19-30, 41` | `FloatingActionBar` shown while `selectionActive`, labelled "Selected screenshots": "{count} selected" in `tabular-nums`, a divider, Clear, then New group, Remove from group (when any selected shot is grouped), Export ZIP, a divider and Delete in red (apps/app-frontend/src/components/ui/screenshots-page/index.vue:1645-1707; packages/ui/src/components/base/FloatingActionBar.vue:19-30, 41) | appears and disappears with the selection; hidden while a modal is open | absent | no selection bar and no bulk actions RE-VERIFIED 2026-11-01: `selection_bar` draws the count and its actions and is called from `draw` (screenshots.rs:2090, called at :1353). | VERIFIED | P0 |
| 304 | screenshots — empty state, illustration and copy | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1519-1524, 230-234`<br>`packages/ui/src/components/base/EmptyState.vue:2-13, 48` | `EmptyState type="no-images"`: `NoImagesIllustration` at `h-[200px] w-auto`, heading `text-2xl font-semibold text-contrast`, description `text-secondary`, `gap-1.5` = 6, container `mx-auto flex flex-col items-center p-6 text-center`; copy "No screenshots yet" / "Screenshots you take in-game will appear here." (apps/app-frontend/src/components/ui/screenshots-page/index.vue:1519-1524, 230-234; packages/ui/src/components/base/EmptyState.vue:2-13, 48) | static | crates/sift-ui/src/pages/screenshots.rs:321-340, 1377-1422 (200pt box, 64pt `Images` glyph, 24pt heading, 16pt body, 6pt gap) | copy, sizes and the 200pt box match; the SVGA illustration is replaced by a 64pt glyph so the block reads far lighter, and the `p-6` breathing room is missing | OPEN | P1 |
| 305 | screenshots — no-results state | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:238-242, 1525-1530` | "No matching screenshots" / "Try a different search or filter." on the same `no-images` illustration (apps/app-frontend/src/components/ui/screenshots-page/index.vue:238-242, 1525-1530) | shown when the filter empties the list | crates/sift-ui/src/pages/screenshots.rs:381-385, 782-792 (same copy, magnifier glyph) | copy matches; illustration differs as above; ours has no live filter, so the state is reachable only through `State::search` | OPEN | P3 |
| 306 | screenshots — error state | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1506-1517, 245` | `EmptyState type="error"` carrying the query's message plus a Retry `Button type="outlined"` (apps/app-frontend/src/components/ui/screenshots-page/index.vue:1506-1517, 245) | shown on a failed query | absent | a failed folder scan is indistinguishable from an empty one | OPEN | P2 |
| 307 | screenshots — loading skeleton | `packages/ui/src/components/base/ReadyTransition.vue:14-24, 68-97`<br>`apps/app-frontend/src/components/ui/screenshots-page/card.vue:227-231` | `ReadyTransition :pending` holds the slot back, rendering a blank `ready-transition-pending` shell while a global loading bar runs, and cross-fades the content in over `duration: 200`; per card the placeholder is `bg-button-bg animate-pulse` (packages/ui/src/components/base/ReadyTransition.vue:14-24, 68-97; apps/app-frontend/src/components/ui/screenshots-page/card.vue:227-231) | 200ms fade-in; infinite pulse until the image decodes | absent — `read_shots` runs synchronously inside the first `draw` (crates/sift-ui/src/pages/screenshots.rs:730-739) | no loading state and no skeleton: a large folder blocks the frame instead of showing a placeholder | OPEN | P2 |
| 308 | screenshots — empty group | `apps/app-frontend/src/components/ui/screenshots-page/group.vue:43-47, 232-238` | "Drag and drop to add screenshots." at `col-span-full m-0 pl-0.5 pt-1 text-base font-base text-secondary opacity-80` (apps/app-frontend/src/components/ui/screenshots-page/group.vue:43-47, 232-238) | static | absent (a group with no shots is never created here) | only reachable once custom groups exist | OPEN | P3 |
| 309 | screenshots — drag and drop, ZIP export | `apps/app-frontend/src/components/ui/screenshots-page/index.vue:1532-1642, 1265-1304, 293-305`<br>`index.vue:692-695` | `DragDropProvider` with a teleported `DragOverlay`, a gather animation at `scale-[0.975] -> 1` over 150ms, and "Export ZIP" writing `{instance} screenshots.zip` / "Modrinth screenshots.zip" (apps/app-frontend/src/components/ui/screenshots-page/index.vue:1532-1642, 1265-1304, 293-305) ‖ file-management.md row 52 — element "Screenshots — Drag a screenshot to another group or instance": Drag-and-drop with a gather animation; dropping on an instance group calls `move_screenshots(keys, targetInstanceId)` (`index.vue:692-695`, `:1281-1360`; `drag-gather.vue`, `use-screenshot-drag-gather.ts`). | 150ms gather; drop outline on a hovered group (apps/app-frontend/src/components/ui/screenshots-page/group.vue:188-192) ‖ file-management.md row 52: — (this report has no states column) | absent ‖ file-management.md row 52: None. | screenshots cannot be dragged between groups or exported ‖ file-management.md row 52: No move between instances. | OPEN | P3 |

## 8. Servers / Hosting

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 310 | pointer — a server card is a link (Enter/Space self-activate) | `UI/components/servers/ServerListing.vue:12-13` | `UI/components/servers/ServerListing.vue:12-13`, card `:16` | — (this report has no states column) | server cards have no press: `SRC/pages/servers.rs:1223-1240` paints sections; the only pointer reads are the `New server` plate `:1201-1210` and the promo dismiss `:1826-1832` | the servers page cannot open a server RE-VERIFIED 2026-11-01: a card is a press target that selects and opens the detail panel (servers.rs:1296-1314, panel :1349-1469, Copy IP :1452-1457), but it is still not a link — no Enter/Space activation and no route to a server page. | OPEN-WITH-CAUSE | P1 |

## 9. Project

Rows 626-648 fold the 23 Project rows of `page-servers-project-about.md` (its rows 26-48) into the table, in the
report's own words and with its own citations, as §10 About still needs. `ours` is the report's cell unchanged, so
where it names a line the page no longer has, the line moved — the `gap` cell's trailing `⇒` says what changed and
what a drive or a test measured.

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 626 | Project — Page frame inset and block gap | apps/app-frontend/src/pages/project/Index.vue:48 | flex flex-col gap-4 p-6 on the page column (apps/app-frontend/src/pages/project/Index.vue:48) | none stated in source | crates/sift-ui/src/pages/project.rs:115 places the tile at rect.min + (0, 8) | page content starts 8pt below the title bar where the reference starts 24pt, and the 16pt gap between header and tabs is replaced by hand-tuned offsets (project.rs:212) ⇒ the page column is p-6 with a 16-point gap: probe reports column 112,72 980x804 and the card at 112,267 | VERIFIED | P0 |
| 627 | Project — Header icon | packages/ui/src/components/project/ProjectPageHeader.vue:8-15 | Avatar size 96px (packages/ui/src/components/project/ProjectPageHeader.vue:8-15) | none stated in source | project.rs:115-116 renders a 64pt tile at radius 12 | the project's identity is drawn at two thirds of the reference size ⇒ the avatar is 96 points (AVATAR, ProjectPageHeader.vue:8-15); a value matched in source, not driven | DONE | P0 |
| 628 | Project — Header title | packages/ui/src/components/base/page-header/index.vue:12-17 | h1 m-0 min-w-0 max-w-full text-2xl font-semibold leading-none text-contrast (packages/ui/src/components/base/page-header/index.vue:12-17) = 24pt, leading-none | none stated in source | project.rs:126-132 sets fonts::title(20.0) | title is 20pt where the reference sets 24pt, and the leading differs (we place the summary at a fixed +26) ⇒ the title is text-2xl semibold with leading-none (TITLE_SIZE/TITLE_LINE) | DONE | P0 |
| 629 | Project — Header summary | page-header/index.vue:20-22, 65 | p m-0 max-w-[44rem] line-clamp-2, body size inherited (page-header/index.vue:20-22, 65) | line-clamp 2 (page-header/index.vue:65) | project.rs:133-139 wraps at rect.width()-320 with fonts::body(13.0) | 13pt against the body 16pt, a wrap width that depends on the window rather than 44rem, and no 2-line clamp ⇒ the summary is body size, max-w-[44rem], clamped to two lines by clamp_to_lines | DONE | P0 |
| 630 | Project — Stats row (downloads, followers) | packages/ui/src/components/base/page-header/metadata/index.vue:2-5<br>metadata/page-header-metadata-item.vue:77-80<br>page-header-metadata-item.vue:3-7 | PageHeaderMetadata: flex flex-wrap items-center gap-x-[1.625rem] gap-y-2 (packages/ui/src/components/base/page-header/metadata/index.vue:2-5); each item icon size-5 text-current + value+label, font-medium leading-none text-secondary, content gap-2 (metadata/page-header-metadata-item.vue:77-80), 26pt BulletDivider between items (page-header-metadata-item.vue:3-7) | hover tooltip with the full number when compact (metadata/page-header-metadata-number-item.vue:56-60) | project.rs:142-157 draws 12pt icons, 12pt text, 14pt gaps, no dividers | icons half size, text too small, spacing 14 instead of 26, dividers missing; the compact/full tooltip pair is absent ⇒ 20-point icons, 16-point labels and the 26-point gap with a bullet between the two counts | DONE | P1 |
| 631 | Project — Category pills | packages/ui/src/components/base/TagItem.vue:18-19<br>ProjectPageHeader.vue:45 | TagItem: px-2 py-1 leading-none rounded-full text-sm border surface-5 bg-button-bg text-secondary, gap-1, svg 16 (packages/ui/src/components/base/TagItem.vue:18-19); tags row hidden below md (ProjectPageHeader.vue:45) | hover underline + active scale 0.95 when the tag is actionable (TagItem.vue:3-4) | project.rs:158-180 draws 11pt text in an 11pt-radius pill on RAISED_BG with a glyph, no border | pills are 3pt too small, unbordered, and not pressable (the reference filters by category) ⇒ the pills are TagItem geometry: px-2 py-1, text-sm, surface-5 border on button-bg | DONE | P1 |
| 632 | Project — Header divider | packages/ui/src/components/base/page-header/index.vue:59-63 | root carries border-0 border-b border-solid border-divider plus pb-4 (packages/ui/src/components/base/page-header/index.vue:59-63) | none stated in source | project.rs has no rule under the header; tabs start at rect.top + 84 (project.rs:212) | the header loses its bottom rule and its 16pt bottom padding ⇒ the header carries a border-b in --color-divider and a 16-point foot | DONE | P2 |
| 633 | Project — Install button | packages/ui/src/components/base/buttons/ButtonFrame.vue:33<br>apps/app-frontend/src/pages/project/Index.vue:138-162 | Button size xl: h-12 gap-2 rounded-2xl px-3.5 text-base font-extrabold leading-5, svg size-6 (packages/ui/src/components/base/buttons/ButtonFrame.vue:33) with DownloadIcon + Install (apps/app-frontend/src/pages/project/Index.vue:138-162) | hover brightness 0.9, active scale 0.97, 150ms ease-out (ButtonFrame.vue:18-21); disabled opacity 50 (ButtonFrame.vue:24) | project.rs:183-203 draws 112x40 at radius 8 with 14pt semibold | button is 8pt short and has the wrong radius, weight and glyph size; no hover/press/disabled state ⇒ geometry matches (probe: 106.6x48 at 985,72); the hover, press and disabled states are not drawn | OPEN | P0 |
| 634 | Project — Kebab menu | apps/app-frontend/src/pages/project/Index.vue:163-170<br>ButtonFrame.vue:41 | TeleportOverflowMenu type=quiet size=xl, label More options (apps/app-frontend/src/pages/project/Index.vue:163-170); icon-only xl is w-12 (ButtonFrame.vue:41) | opening menu is a hover/click state on a 48pt target | project.rs:204-208 draws a 12pt-shrunk glyph in a 24pt box with no target | the target is a quarter of the reference's area and the menu does not open ⇒ the target is 48 points (probe: 929,72 48x48); the menu it opens is not built | OPEN | P0 |
| 635 | Project — Install button states | project/Index.vue:119-162 | installed -> CheckIcon + Installed (disabled) with tooltip Already installed; switch -> SwapIcon + Switch version; install -> colored brand; validating/installing/selected labels (project/Index.vue:119-162) | disabled + tooltip on hover when installed (Index.vue:120-128, 140-142) | project.rs:183-203 always paints Download + Install | none of the four label/icon states exist, so an installed project still offers Install ⇒ the four install states need the installer, which the page cannot reach | OPEN | P1 |
| 636 | Project — Tab strip | packages/ui/src/components/base/NavTabs.vue:7-13<br>NavTabs.vue:24<br>NavTabs.vue:63-72 | NavTabs (mode=navigation): nav relative flex w-fit rounded-full bg-bg-raised p-1 text-xs sm:text-sm font-bold (packages/ui/src/components/base/NavTabs.vue:7-13); each link flex items-center gap-2 px-4 py-2 (NavTabs.vue:24) = 36pt pill; slider div bg-button-bgSelected (NavTabs.vue:63-72) | slider left/right/top/bottom 80ms cubic-bezier(0.4,0,0.2,1) with a 175ms stagger between edges (NavTabs.vue:260-273, 366-373); label colour 100ms, delayed 100ms on change (NavTabs.vue:375-381) | project.rs:211-237 draws its own pills: 30pt tall, 13pt semibold, radius 15, 4pt apart, no raised track and no slider | no rounded track around the tabs, pills 6pt short, and switching snaps with no 80ms slide or colour delay ⇒ the track is 289.5x44 at 112,207 with three 36-point pills; the 80 ms slide is implemented, unseen | VERIFIED | P0 |
| 637 | Project — Which tabs exist | apps/app-frontend/src/pages/project/Index.vue:174-192 | Description always; Versions unless projectV3.minecraft_server is set; Gallery only while gallery.length > 0 (apps/app-frontend/src/pages/project/Index.vue:174-192) | tabs appear/disappear with data; NavTabs renders nothing under 2 links (NavTabs.vue:3) | project.rs:216 iterates all three unconditionally (project.rs:77) | a project with no gallery still offers a Gallery tab that says No images yet ⇒ tabs() offers Versions unless it is a server project and Gallery only while the gallery has images | DONE | P2 |
| 638 | Project — Tab content card | packages/assets/styles/classes.scss:499-508, 534-537 | Card -> .base-card: padding 1rem, background var(--surface-3), border-radius var(--radius-lg)=16pt, border 1px solid var(--surface-4), margin-bottom var(--gap-md)=12pt (packages/assets/styles/classes.scss:499-508, 534-537) | none stated in source | project.rs:240-244 fills RAISED_BG at radius 12 with no inner padding (text is inset 32 by hand, project.rs:337, 380) | card is 4pt too tight and has no border; the 16pt padding is replaced by ad-hoc 16/32 offsets ⇒ the card is 112,267 with radius 16 and a 1-point surface-4 border | VERIFIED | P1 |
| 639 | Project — Description tab contents | apps/app-frontend/src/pages/project/Description.vue:1-5; packages/ui/src/components/project/ProjectPageDescription.vue:2 | Card wrapping only the rendered markdown body (apps/app-frontend/src/pages/project/Description.vue:1-5; packages/ui/src/components/project/ProjectPageDescription.vue:2) | none stated in source | project.rs:334-368 draws an action row of three invented buttons (How to install / Discord / Rate) above the body | invented controls on the first screen of the page, in a tab that has none ⇒ the invented action row is deleted, not hidden | DONE | P0 |
| 640 | Project — Description body type | classes.scss:500, 845-849 | markdown-body inside the card's 16pt padding, at the page's body size (classes.scss:500, 845-849) | none stated in source | project.rs:370-385: fonts::body(13.0), first line at card.left+32, paragraphs separated by a hard 12pt | body is 13pt against 16pt and inset 32 instead of 16 ⇒ the body is 16 points at the card's own 16-point inset, paragraphs 12 apart | DONE | P1 |
| 641 | Project — Versions tab: structure | packages/ui/src/components/base/Table.vue:2-8, 24-36<br>project/ProjectPageVersions.vue:560-615 | Table: overflow-hidden rounded-2xl border border-surface-4, header row bg-surface-3 with h-12 cells, p-4 slot header (packages/ui/src/components/base/Table.vue:2-8, 24-36); columns channel 4.5rem, Version, Game version, Platform, [Environment], Published 12%, Downloads 12%, actions 1% with py-3 cells (project/ProjectPageVersions.vue:560-615) | row hover/click when versionLink is set (ProjectPageVersions.vue:49-60); 20 items per page (ProjectPageVersions.vue:693) | project.rs:254-286 draws 48pt rows on SURFACE_1 with one bold number and one 11pt detail line | no table, no header, no columns, no pagination and no row actions ⇒ a real table: h-12 header, h-14 rows with their own top rules, the sheet's seven columns | VERIFIED | P0 |
| 642 | Project — Versions tab: filters and paging | project/ProjectPageVersions.vue:2-28 | VersionFilterControl above the table plus Pagination in the same row, flex flex-wrap justify-between gap-2 mb-3 (project/ProjectPageVersions.vue:2-28) | filter chips toggle on click (ProjectPageVersions.vue:61-70, 123-137) | nothing between the card top and the first row (project.rs:259-261) | no way to narrow or page the list ⇒ the filter chips exist now: one per channel and per loader the project carries, toggled on and off, OR inside a dimension and AND across them, driven (`Release` left two rows of four). The pager is still missing: four sample versions fit on one page | OPEN | P1 |
| 643 | Project — Versions tab: cell contents | ProjectPageVersions.vue:61-71, 82-88, 123-137, 238-256 | channel indicator (VersionChannelIndicator) alone in its cell; version_number font-medium text-contrast; game versions as TagItem chips truncated to 12rem; published as relative time; downloads compact (ProjectPageVersions.vue:61-71, 82-88, 123-137, 238-256) | hover tooltip carries the full date and download count (ProjectPageVersions.vue:239-240, 248-249) | project.rs:277-283 prints Fabric + game versions + channel + a long date in one 11pt string | five columns collapsed into one string, with a loader that the reference keeps in its own column and an absolute date where the reference shows relative time ⇒ channel dot, version number, game-version and platform chips, relative published, compact downloads | DONE | P1 |
| 644 | Project — Gallery grid | apps/app-frontend/src/pages/project/Gallery.vue:91-96<br>Gallery.vue:3-15, 105-120 | grid-template-columns repeat(auto-fill, minmax(20rem, 1fr)), gap 1rem (apps/app-frontend/src/pages/project/Gallery.vue:91-96); image aspect-ratio 2/1 object-fit cover; body p-4 with title and description; CalendarIcon + date in the foot (Gallery.vue:3-15, 105-120) | image click opens the viewer (Gallery.vue:4-6) | project.rs:292-332 fixes two columns, uses a 9/16 tile (16:9), draws only the title under it | aspect ratio is 16:9 against the reference's 2:1, columns do not reflow with width, description and date missing ⇒ 466x381 items at 128,283 and 610,283, pictures exactly 466x233, decoded | VERIFIED | P1 |
| 645 | Project — Gallery image viewer | apps/app-frontend/src/pages/project/Gallery.vue:17-33 | ImageViewerEditor with an Open image in new tab action per item (apps/app-frontend/src/pages/project/Gallery.vue:17-33) | click opens the viewer at the pressed index (Gallery.vue:74-80); navigation tracked per image | no viewer in project.rs:292-332 | images cannot be enlarged ⇒ a press opens the picture full-page over a scrim; Escape returns (pixel checks both ways) | VERIFIED | P2 |
| 646 | Project / sidebar — Compatibility block type | packages/ui/src/components/project/ProjectSidebarCompatibility.vue:3<br>ProjectSidebarCompatibility.vue:5-7<br>ProjectSidebarCompatibility.vue:8<br>ProjectSidebarCompatibility.vue:2 | h2 text-lg m-0 (18pt) (packages/ui/src/components/project/ProjectSidebarCompatibility.vue:3); h3 text-primary !font-normal text-base m-0 (16pt normal) (ProjectSidebarCompatibility.vue:5-7); chips flex flex-wrap gap-1 (ProjectSidebarCompatibility.vue:8); whole block only when project.versions is non-empty (ProjectSidebarCompatibility.vue:2) | none stated in source | project.rs:407-437: 15pt semibold heading, 12pt secondary subtitle, 11pt chips radius 6 with 6pt gaps, drawn unconditionally | every level is 2-4pt small and the chip row is 2pt looser; the empty-versions guard is missing ⇒ the compatibility block is 18-point heading, 16-point normal subtitle, chips at gap 1 | DONE | P1 |
| 647 | Project / sidebar — Sidebar sections are cards | packages/ui/src/components/page/SidebarCard.vue:11-18 | each section renders SidebarCard: flex flex-col gap-3 p-4 pt-3, card-shadow mb-4 rounded-2xl bg-surface-3 border border-solid border-surface-4, h2 text-lg font-semibold (packages/ui/src/components/page/SidebarCard.vue:11-18) | none stated in source | project.rs:404-562 draws Compatibility, Links, Creators and Details as one flat flow of 15pt headings on the panel's own background | the four panels lose their raised card, border and 12pt separation, so the sidebar reads as one undifferentiated list ⇒ the four sections are SidebarCards now — rounded-2xl surface-3 with a surface-4 border, p-4 pt-3, a gap-3 under the 18-point heading and 16 between cards — with the rows placed by a wrapping pass, pinned by a test that drives the panel through a real egui context (a full project claims more than the shell's 400-point block; a card-only one claims nothing) | DONE | P1 |
| 648 | Project / sidebar — Sidebar default content is replaced, not appended | apps/app-frontend/src/app/shell/sidebar/index.vue:106-107, 165-167<br>pages/project/Index.vue:3-47 | when a page teleports into #sidebar-teleport-target the default stack (checklist, accounts, friends, news) is hidden by .sidebar-teleport-content:empty + .sidebar-default-content.sidebar-enabled { display: contents } (apps/app-frontend/src/app/shell/sidebar/index.vue:106-107, 165-167); the project teleports (pages/project/Index.vue:3-47) | none stated in source | crates/sift-ui/src/shell.rs:1542-1552 draws the project's panel block and then continues the shared stack from below it | on a project page we show the project's sections plus getting-started/news/hosting, where the reference shows only the project's ⇒ the shell draws the project's panel and then the shared stack; that is shell.rs's seam, not this page's | OPEN | P0 |

## 10. About

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|

## 11. Design tokens

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 311 | surfaces — --surface-1 | `packages/assets/styles/variables.scss:2`<br>`packages/assets/styles/variables.scss:233`<br>`packages/assets/styles/variables.scss:412` | #ebebeb (light) packages/assets/styles/variables.scss:2; #16181c (dark) packages/assets/styles/variables.scss:233; #000000 (oled) packages/assets/styles/variables.scss:412 | static ground under --color-bg | theme.rs:65 (light theme.rs:638, dark theme.rs:465) | carried; light and dark match; the oled override at theme.rs:612 sets #0d0d0d where the sheet says #000000 | OPEN | P1 |
| 312 | surfaces — --surface-1-5 | `packages/assets/styles/variables.scss:3`<br>`packages/assets/styles/variables.scss:234`<br>`packages/assets/styles/variables.scss:413` | #ededed packages/assets/styles/variables.scss:3; #1a1c20 packages/assets/styles/variables.scss:234; #050506 packages/assets/styles/variables.scss:413 | static | theme.rs:71 (light theme.rs:640, dark theme.rs:467) | carried; light and dark match; oled inherits dark #1a1c20 instead of #050506 | OPEN | P2 |
| 313 | surfaces — --surface-2 | `packages/assets/styles/variables.scss:4`<br>`packages/assets/styles/variables.scss:235`<br>`packages/assets/styles/variables.scss:414` | #f5f5f5 packages/assets/styles/variables.scss:4; #1d1f23 packages/assets/styles/variables.scss:235; #09090a packages/assets/styles/variables.scss:414 | static; a recessed well or an input ground | theme.rs:77 (light theme.rs:642, dark theme.rs:469) | carried; light and dark match; oled inherits dark instead of #09090a; read only at pages/screenshots.rs:1117 | OPEN | P2 |
| 314 | surfaces — --surface-2-5 | `packages/assets/styles/variables.scss:5`<br>`packages/assets/styles/variables.scss:236`<br>`packages/assets/styles/variables.scss:415` | #eef1f5 packages/assets/styles/variables.scss:5; #222429 packages/assets/styles/variables.scss:236; #0c0d11 packages/assets/styles/variables.scss:415 | static | theme.rs:83 (light theme.rs:644, dark theme.rs:471) | carried; light and dark match; oled inherits dark instead of #0c0d11; never read by any page | OPEN | P2 |
| 315 | surfaces — --surface-3 | `packages/assets/styles/variables.scss:6`<br>`packages/assets/styles/variables.scss:237`<br>`packages/assets/styles/variables.scss:416` | #f8f8f8 packages/assets/styles/variables.scss:6; #27292e packages/assets/styles/variables.scss:237; #101013 packages/assets/styles/variables.scss:416 | static; the ground of --color-raised-bg, i.e. the title bar and rail (ref-home.png at (5,5) is #27292e) | theme.rs:89 (light theme.rs:646, dark theme.rs:473) | carried; light and dark match; oled inherits dark instead of #101013; one use at pages/screenshots.rs:1121 | OPEN | P2 |
| 316 | surfaces — --surface-4 | `packages/assets/styles/variables.scss:7`<br>`packages/assets/styles/variables.scss:238`<br>`packages/assets/styles/variables.scss:417` | #ffffff packages/assets/styles/variables.scss:7; #34363c packages/assets/styles/variables.scss:238; #1b1b20 packages/assets/styles/variables.scss:417 | static; a control fill (--color-button-bg resolves here) and the inset ring of button-frame--base (packages/ui/src/components/base/buttons/ButtonFrame.vue:138) | theme.rs:95 (light theme.rs:648, dark theme.rs:475) | carried; light and dark match; oled inherits dark #34363c where the sheet says #1b1b20; read at pages/screenshots.rs:921 and :1006 only | OPEN | P1 |
| 317 | surfaces — --surface-5 | `packages/assets/styles/variables.scss:8`<br>`packages/assets/styles/variables.scss:239`<br>`packages/assets/styles/variables.scss:418` | #dddddd packages/assets/styles/variables.scss:8; #42444a packages/assets/styles/variables.scss:239; #25262b packages/assets/styles/variables.scss:418 | static; the 1 px inset ring on every base button and on the kbd keycap (apps/app-frontend/src/components/ui/WelcomeScreen.vue:118 border-surface-5; packages/ui/src/components/base/buttons/ButtonFrame.vue:138) | theme.rs:101 (light theme.rs:650, dark theme.rs:477) | carried and correct but no control uses it: home.rs:566-570 strokes BUTTON_BORDER where the sheet says surface-5, and the comment two lines above says surface-5 | OPEN | P0 |
| 318 | surfaces — --color-bg | `packages/assets/styles/variables.scss:94` | var(--surface-1) packages/assets/styles/variables.scss:94, :325, :412 | static page ground; ref-home.png (700,450) is #16181c | theme.rs:51 (light theme.rs:632, dark theme.rs:459) | carried, matches, and is what colors() paints (theme.rs:1001-1002); the oled override to #000000 at theme.rs:610 is correct | DONE | P0 |
| 319 | surfaces — --color-raised-bg | `packages/assets/styles/variables.scss:95` | var(--surface-3) packages/assets/styles/variables.scss:95, :326, :416 | static; the title bar and rail, and every card | theme.rs:53 (light theme.rs:634, dark theme.rs:461) | carried; light and dark match; the oled override at theme.rs:611 is #080808 where the sheet says surface-3 = #101013 | OPEN | P1 |
| 320 | surfaces — --color-super-raised-bg | `packages/assets/styles/variables.scss:96` | var(--surface-4) packages/assets/styles/variables.scss:96, :327, :417 | static; the topmost raised layer | theme.rs:59 (light theme.rs:636, dark theme.rs:463) | carried; the oled override at theme.rs:613 is #121212 where the sheet says #1b1b20 | OPEN | P1 |
| 321 | text — --color-text-primary | `packages/assets/styles/variables.scss:88`<br>`packages/assets/styles/variables.scss:319` | #1a202c packages/assets/styles/variables.scss:88; #ffffff packages/assets/styles/variables.scss:319 | headings and hovered labels; the ink --color-contrast resolves to | theme.rs:156 (light theme.rs:675, dark theme.rs:502) | carried, both values match | DONE | P2 |
| 322 | text — --color-text-default | `packages/assets/styles/variables.scss:89`<br>`packages/assets/styles/variables.scss:320` | #2c2e31 packages/assets/styles/variables.scss:89; #b0bac5 packages/assets/styles/variables.scss:320 | body text; --color-base resolves here | theme.rs:162 (light theme.rs:677, dark theme.rs:504) | carried, both values match | DONE | P2 |
| 323 | text — --color-text-tertiary | `packages/assets/styles/variables.scss:90`<br>`packages/assets/styles/variables.scss:321` | #484d54 packages/assets/styles/variables.scss:90; #96a2b0 packages/assets/styles/variables.scss:321 | muted text; --color-secondary resolves here | theme.rs:168 (light theme.rs:679, dark theme.rs:506) | carried, both values match | DONE | P2 |
| 324 | text — --color-base | `packages/assets/styles/variables.scss:105` | var(--color-text-default) packages/assets/styles/variables.scss:105, :336 | default ink; packages/assets/styles/defaults.scss:11 sets body colour to it | theme.rs:136 (light theme.rs:667, dark theme.rs:494) | carried and matches; read 52 times (e.g. pages/home.rs:447) | DONE | P0 |
| 325 | text — --color-secondary | `packages/assets/styles/variables.scss:106` | var(--color-text-tertiary) packages/assets/styles/variables.scss:106, :337 | muted and hint text; e.g. apps/app-frontend/src/components/ui/WelcomeScreen.vue:114 and :129 use text-secondary | theme.rs:142 (light theme.rs:669, dark theme.rs:496) | carried and matches; read 118 times, the second most used text token | DONE | P0 |
| 326 | text — --color-contrast | `packages/assets/styles/variables.scss:107` | var(--color-text-primary) packages/assets/styles/variables.scss:107, :338 | headings and labels; e.g. apps/app-frontend/src/components/ui/library/index.vue:268 and packages/ui/src/components/base/page-header/apps/app-frontend/src/components/ui/library/index.vue:13 use text-contrast | theme.rs:144 (light theme.rs:671, dark theme.rs:498) | carried and matches; read 89 times, the most used text token | DONE | P0 |
| 327 | text — --color-accent-contrast | `packages/assets/styles/variables.scss:108`<br>`packages/assets/styles/variables.scss:339` | #ffffff packages/assets/styles/variables.scss:108; #000000 packages/assets/styles/variables.scss:339 | ink on a brand or hue fill; the welcome CTA label (home.rs:453) | theme.rs:150 (light theme.rs:673, dark theme.rs:500) | carried, both values match; read 37 times | DONE | P0 |
| 328 | text — --dark-color-base / --dark-color-contrast | `packages/assets/styles/variables.scss:192-193` | #b0bac5 / #ecf9fb (packages/assets/styles/variables.scss:192-193, inside the html block) | static; no state | absent: no field in theme.rs:47-441 | not carried; nothing in the reference reads either property, so both are defined and unused | NOT-IN-SOURCE | P2 |
| 329 | brand — --color-brand | `packages/assets/styles/variables.scss:123`<br>`packages/assets/styles/variables.scss:354` | var(--color-green-600) #00af5c packages/assets/styles/variables.scss:123 via :47; dark var(--color-green-500) #1bd96a packages/assets/styles/variables.scss:354 via :278 | rest fill of the primary CTA (apps/app-frontend/src/components/ui/WelcomeScreen.vue:105 type=colored color=brand); hover multiplies it by --hover-brightness (packages/ui/src/components/base/buttons/ButtonFrame.vue:21) | theme.rs:173 (light theme.rs:682, dark theme.rs:509) | carried, both values match; read 46 times; the hover is painted as a settled colour rather than a filter tween | DONE | P0 |
| 330 | brand — --color-brand-highlight | `packages/assets/styles/variables.scss:124`<br>`packages/assets/styles/variables.scss:355` | rgba(0,175,92,0.25) packages/assets/styles/variables.scss:124 via :112; dark rgba(27,217,106,0.25) packages/assets/styles/variables.scss:355 | selection and selected rows; --color-button-bg-selected in dark (packages/assets/styles/variables.scss:376); text selection at theme.rs:1030 | theme.rs:175 (light theme.rs:684, dark theme.rs:511) | carried and matches (premultiplied, alpha 64); one call site, pages/servers.rs:1836 | DONE | P1 |
| 331 | brand — --color-brand-shadow | `packages/assets/styles/variables.scss:125`<br>`packages/assets/styles/variables.scss:356` | rgba(0,175,92,0.7) packages/assets/styles/variables.scss:125; dark rgba(27,217,106,0.7) packages/assets/styles/variables.scss:356 | the focus-visible ring: focus-visible ring-4 ring-brand-shadow (packages/ui/src/components/base/buttons/ButtonFrame.vue:23) | theme.rs:181 (light theme.rs:686, dark theme.rs:513) | carried and matches, never painted: sift-ui draws no focus ring anywhere, and FOCUS_RING is unused too | DONE | P1 |
| 332 | radix — --color-red-50..950 | `packages/assets/styles/variables.scss:10-19`<br>`packages/assets/styles/variables.scss:241-250` | #fef2f2 .. #490819 (packages/assets/styles/variables.scss:10-19); #fff0f1 .. #50011a (packages/assets/styles/variables.scss:241-250) | static ramp; only the alias is used at rest | absent: only the --color-red alias is carried, at theme.rs:186 | all 66 step tokens are absent while packages/tooling-config/tailwind/tailwind-preset.ts:28-41 exposes every step as a utility | OPEN | P1 |
| 333 | radix — --color-orange-50..950 | `packages/assets/styles/variables.scss:23-33`<br>`packages/assets/styles/variables.scss:254-264` | #fdf8ed .. #3f1509 (packages/assets/styles/variables.scss:23-33); #fff8ed .. #441606 (packages/assets/styles/variables.scss:254-264) | static ramp | absent: only the alias at theme.rs:188 | steps absent; exposed at packages/tooling-config/tailwind/tailwind-preset.ts:42-55 | OPEN | P1 |
| 334 | radix — --color-green-50..950 | `packages/assets/styles/variables.scss:36-46`<br>`packages/assets/styles/variables.scss:267-277` | #eefff6 .. #00341d (packages/assets/styles/variables.scss:36-46); #effef5 .. #033018 (packages/assets/styles/variables.scss:267-277) | static ramp | absent: only the alias at theme.rs:194 | steps absent; exposed at packages/tooling-config/tailwind/tailwind-preset.ts:56-69 | OPEN | P1 |
| 335 | radix — --color-blue-50..950 | `packages/assets/styles/variables.scss:49-59`<br>`packages/assets/styles/variables.scss:280-290` | #f2f7fd .. #112340 (packages/assets/styles/variables.scss:49-59); #eef6ff .. #152356 (packages/assets/styles/variables.scss:280-290) | static ramp | absent: only the alias at theme.rs:200 | steps absent; exposed at packages/tooling-config/tailwind/tailwind-preset.ts:70-83 | OPEN | P1 |
| 336 | radix — --color-purple-50..950 | `packages/assets/styles/variables.scss:62-72`<br>`packages/assets/styles/variables.scss:293-303` | #faf5ff .. #370368 (packages/assets/styles/variables.scss:62-72); #faf5ff .. #3d0566 (packages/assets/styles/variables.scss:293-303) | static ramp; --color-ad-highlight resolves to --color-purple (packages/assets/styles/variables.scss:214) | absent: only the alias at theme.rs:206 | steps absent; exposed at packages/tooling-config/tailwind/tailwind-preset.ts:84-97 | OPEN | P1 |
| 337 | radix — --color-gray-50..950 | `packages/assets/styles/variables.scss:75-85`<br>`packages/assets/styles/variables.scss:306-316` | #f5f5f6 .. #252627 (packages/assets/styles/variables.scss:75-85); #f5f7f8 .. #303136 (packages/assets/styles/variables.scss:306-316) | static ramp | absent: only the alias at theme.rs:212 | steps absent; exposed at packages/tooling-config/tailwind/tailwind-preset.ts:98-111 | OPEN | P1 |
| 338 | radix — the six hue aliases | `packages/assets/styles/variables.scss:21` | light red-600 #cb2245 packages/assets/styles/variables.scss:21, orange-500 #e08325 :34, green-600 #00af5c :47, blue-600 #1f68c0 :60, purple-600 #8e32f3 :73, gray-600 #595b61 :86; dark #ff496e :252, #ffa347 :265, #1bd96a :278, #4f9cff :291, #c78aff :304, #9fa4b3 :317 | static; badge inks only | theme.rs:186-212 (light theme.rs:689-699, dark theme.rs:516-526) | all six carried and all twelve values match; only RED is read (3 sites, e.g. shell.rs:679) | DONE | P2 |
| 339 | wash — --color-red/orange/green/blue/purple/gray-highlight | `packages/assets/styles/variables.scss:110-115`<br>`packages/assets/styles/variables.scss:341-346` | light rgba(203,34,69,.25) .. rgba(89,91,97,.25) (packages/assets/styles/variables.scss:110-115); dark rgba(255,73,110,.25) .. rgba(159,164,179,.25) (packages/assets/styles/variables.scss:341-346) | static wash behind a badge or a selected row; no transition of its own | theme.rs:221-251 (light theme.rs:702-712, dark theme.rs:529-539) | all six carried and all twelve values match (alpha 64); none is read | DONE | P2 |
| 340 | wash — --color-red/orange/green/blue/purple-bg | `packages/assets/styles/variables.scss:117-121`<br>`packages/assets/styles/variables.scss:348-352` | light rgba(..,0.1) (packages/assets/styles/variables.scss:117-121); dark rgba(..,0.2) (packages/assets/styles/variables.scss:348-352) | static banner background | theme.rs:257-281 (light theme.rs:714-722, dark theme.rs:541-549) | all five carried and match (alpha 26 light, 51 dark); none is read | DONE | P2 |
| 341 | chrome — --color-button-bg | `packages/assets/styles/variables.scss:98`<br>`packages/assets/styles/variables.scss:417` | var(--surface-4) packages/assets/styles/variables.scss:98, :329; in oled that is #1b1b20 via packages/assets/styles/variables.scss:417 | rest fill of a base button and of an input (packages/ui/src/components/base/buttons/ButtonFrame.vue:45 bg-surface-4); hover keeps the fill and adds brightness (packages/ui/src/components/base/buttons/ButtonFrame.vue:21, :57) | theme.rs:106 (light theme.rs:653, dark theme.rs:480) | carried, light and dark match, read 36 times; oled inherits dark #34363c where the sheet says #1b1b20 | OPEN | P1 |
| 342 | chrome — --color-button-border | `packages/assets/styles/variables.scss:99`<br>`packages/assets/styles/variables.scss:330` | rgba(161,161,161,0.35) packages/assets/styles/variables.scss:99; rgba(193,190,209,0.12) packages/assets/styles/variables.scss:330 | the edge of a popover and of an input; NOT the edge of button-frame--base, which is border-0 (packages/ui/src/components/base/buttons/ButtonFrame.vue:16) and carries a surface-5 inset ring instead (packages/ui/src/components/base/buttons/ButtonFrame.vue:137-139) | theme.rs:108 (light theme.rs:655, dark theme.rs:482) | carried and correct, but substituted for the surface-5 ring at 5 sites (home.rs:533, home.rs:569, home.rs:921 area, servers.rs:1688, screenshots.rs) | OPEN | P1 |
| 343 | chrome — --color-button-bg-selected | `packages/assets/styles/variables.scss:156`<br>`packages/assets/styles/variables.scss:376` | var(--color-brand) packages/assets/styles/variables.scss:156; dark var(--color-brand-highlight) packages/assets/styles/variables.scss:376 | selected button or tab fill; the search-kind pill (shell.rs:1237) | theme.rs:114 (light theme.rs:657, dark theme.rs:484) | carried, both values match; read 14 times | DONE | P0 |
| 344 | chrome — --color-button-text-selected | `packages/assets/styles/variables.scss:157`<br>`packages/assets/styles/variables.scss:377` | var(--color-accent-contrast) packages/assets/styles/variables.scss:157; dark var(--color-brand) packages/assets/styles/variables.scss:377 | ink on a selected button | theme.rs:120 (light theme.rs:659, dark theme.rs:486) | carried, both values match; read 18 times | DONE | P0 |
| 345 | chrome — --color-divider | `packages/assets/styles/variables.scss:102`<br>`packages/assets/styles/variables.scss:333` | var(--surface-5) packages/assets/styles/variables.scss:102; dark var(--color-button-bg) -> var(--surface-4) #34363c packages/assets/styles/variables.scss:333 | static rule; the page header's own border-b border-divider (packages/ui/src/components/base/page-header/apps/app-frontend/src/components/ui/library/index.vue:60) | theme.rs:125 (light theme.rs:662, dark theme.rs:489) | carried and matches; read 12 times; inside the sidebar it has to be --brand-gradient-border instead (packages/assets/styles/variables.scss:373, apps/app-frontend/src/app/shell/sidebar/index.vue:161) | DONE | P0 |
| 346 | chrome — --color-divider-dark | `packages/assets/styles/variables.scss:103`<br>`packages/assets/styles/variables.scss:334` | var(--surface-5) #dddddd packages/assets/styles/variables.scss:103; #646c75 packages/assets/styles/variables.scss:334 | static heavy rule | theme.rs:131 (light theme.rs:664, dark theme.rs:491) | carried and matches, never read | DONE | P2 |
| 347 | chrome — --color-scrollbar | `packages/assets/styles/variables.scss:100`<br>`packages/assets/styles/variables.scss:331` | #96a2b0 packages/assets/styles/variables.scss:100; var(--surface-5) #42444a packages/assets/styles/variables.scss:331 | static; ::-webkit-scrollbar-thumb (packages/assets/styles/defaults.scss:148-150) and scrollbar-color (packages/assets/styles/defaults.scss:155) | theme.rs:307 (light theme.rs:732, dark theme.rs:559) | carried and matches, never read: egui exposes no scrollbar colour, so the thumb is painted by the toolkit | DONE | P1 |
| 348 | chrome — --color-tooltip-bg | `packages/assets/styles/variables.scss:209` | var(--surface-3) packages/assets/styles/variables.scss:209 | static | theme.rs:313 (light theme.rs:734, dark theme.rs:561) | carried and matches, never read; the rail's label pill paints theme.CONTRAST instead (shell.rs:450-456) | DONE | P1 |
| 349 | chrome — --color-tooltip-text | `packages/assets/styles/variables.scss:208` | var(--color-contrast) packages/assets/styles/variables.scss:208 | static | theme.rs:319 (light theme.rs:736, dark theme.rs:563) | carried and matches, never read | DONE | P1 |
| 350 | chrome — --color-focus-ring | `packages/assets/styles/variables.scss:222` | #ea80ff packages/assets/styles/variables.scss:222 (declared once in the html block, so both schemes share it) | focus-visible only; nothing in the shell draws one | theme.rs:325 (light theme.rs:738, dark theme.rs:565) | carried and matches, never read; the reference's focus is outline 0.2s (packages/assets/styles/classes.scss:197) plus ring-4 brand-shadow (packages/ui/src/components/base/buttons/ButtonFrame.vue:23) | DONE | P1 |
| 351 | chrome — --color-link / -hover / -active | `packages/assets/styles/variables.scss:216-218` | var(--color-blue) each packages/assets/styles/variables.scss:216-218; the sheet marks hover and active DEPRECATED at :217-218 | hover and active are declared and both resolve to the same blue | theme.rs:286-298 (light theme.rs:725-729, dark theme.rs:552-556) | all three carried and match; only LINK is read (pages/project.rs:488) | DONE | P2 |
| 352 | chrome — --hover-brightness | `packages/assets/styles/variables.scss:187`<br>`packages/assets/styles/variables.scss:407` | 0.9 packages/assets/styles/variables.scss:187; 1.25 packages/assets/styles/variables.scss:407 | the hover filter: 0.2s ease-in-out (packages/assets/styles/classes.scss:195) in the old family, 150ms ease-out (packages/ui/src/components/base/buttons/ButtonFrame.vue:18) in the one the app uses | theme.rs:436 (light theme.rs:775, dark theme.rs:602) | carried and matches; applied through Theme::hovered (theme.rs:1087-1093) to hover AND active, where .button-base uses 0.85 and 0.8 (packages/assets/styles/classes.scss:215, :220) | OPEN | P1 |
| 353 | chrome — --color-gradient-button-bg | `packages/assets/styles/variables.scss:159`<br>`packages/assets/styles/variables.scss:379`<br>`packages/assets/styles/variables.scss:433` | linear-gradient(180deg,#f8f9fa 0%,#dce0e6 100%) packages/assets/styles/variables.scss:159; (180deg,#3a3d47,#33363d) packages/assets/styles/variables.scss:379; oled (180deg,#1b1b20,#25262b) packages/assets/styles/variables.scss:433 | static fill of a gradient button | absent: no field in theme.rs:47-441 | not carried; reachable only through the Tailwind class bg-gradient-button, which no file in this tree uses | OPEN | P2 |
| 354 | chrome — --loading-bar-gradient | `packages/assets/styles/variables.scss:161`<br>`packages/assets/styles/variables.scss:381` | linear-gradient(to right, var(--color-brand) 0%, #00af5c 100%) packages/assets/styles/variables.scss:161; dark to #1ffa9a packages/assets/styles/variables.scss:381 | indeterminate fill, left to right | absent | not carried; used by packages/ui/src/components/base/LoadingBar.vue:27 and packages/ui/src/components/base/PageLoadingBar.vue:160 | OPEN | P2 |
| 355 | gradient — --brand-gradient-bg | `packages/assets/styles/variables.scss:142-146`<br>`packages/assets/styles/variables.scss:370`<br>`packages/assets/styles/variables.scss:422-426` | light linear-gradient(0deg, rgba(68,182,138,0.175) 0%, rgba(58,250,112,0.125) 100%) packages/assets/styles/variables.scss:142-146; dark linear-gradient(0deg, rgba(14,35,19,0.2) 0%, rgba(55,137,73,0.1) 100%) packages/assets/styles/variables.scss:370; oled rgba(22,66,51,0.15) to rgba(55,137,73,0.1) packages/assets/styles/variables.scss:422-426 | static; the .app-sidebar ground (apps/app-frontend/src/app/shell/sidebar/index.vue:156). ref-home.png (1435,450) is #171f1e, which is the dark stops over #16181c | no theme field; hardcoded dark triplets at panel.rs:353-358 | not carried as a token: panel.rs paints (55,137,73,0.10) to (14,35,19,0.20) whatever the scheme is, so Light and OLED get the dark sidebar | OPEN | P1 |
| 356 | gradient — --brand-gradient-strong-bg | `packages/assets/styles/variables.scss:147-151`<br>`packages/assets/styles/variables.scss:371`<br>`packages/assets/styles/variables.scss:427-431` | light linear-gradient(270deg, rgba(68,182,138,0.175) 0%, rgba(36,225,91,0.12) 100%) packages/assets/styles/variables.scss:147-151; dark linear-gradient(270deg, #09110d 10%, #131f17 100%) packages/assets/styles/variables.scss:371; oled rgba(9,18,14,0.6) to rgba(19,31,23,0.5) packages/assets/styles/variables.scss:427-431 | static | absent | not carried and not painted anywhere | OPEN | P2 |
| 357 | gradient — --brand-gradient-button | `packages/assets/styles/variables.scss:152`<br>`packages/assets/styles/variables.scss:372` | rgba(255,255,255,0.5) packages/assets/styles/variables.scss:152; dark rgba(255,255,255,0.08) packages/assets/styles/variables.scss:372 | static; it is what --surface-4 and --color-button-bg become inside .app-sidebar (apps/app-frontend/src/app/shell/sidebar/index.vue:157-158) | absent: no field, and panel.rs re-scopes no token | not carried: every card and button inside the sidebar is painted with surface-4 / BUTTON_BG (#34363c in dark) instead of an 8% white wash over the sidebar ground | OPEN | P0 |
| 358 | gradient — --brand-gradient-border | `packages/assets/styles/variables.scss:153`<br>`packages/assets/styles/variables.scss:373` | rgba(32,64,32,0.15) packages/assets/styles/variables.scss:153; dark rgba(155,255,160,0.08) packages/assets/styles/variables.scss:373 | static; becomes --surface-5, --color-divider and --color-divider-dark inside .app-sidebar (apps/app-frontend/src/app/shell/sidebar/index.vue:160-162), and is the sidebar's own border-l (apps/app-frontend/src/app/shell/sidebar/index.vue:89) | absent | not carried: sidebar section rules and the sidebar's left border are painted with DIVIDER or BUTTON_BORDER instead of the green-tinted token | OPEN | P0 |
| 359 | gradient — sidebar ground and tinted card, measured | `packages/assets/styles/variables.scss:370` | ref-home.png (1435,450) is #171f1e under --brand-gradient-bg packages/assets/styles/variables.scss:370; (1200,450) is #1e3b2d inside a brand-tinted sidebar card | static at rest | ours-home4.png (1435,450) is #171f1e and matches; (1200,450) is #161e1d, flat | the ground matches and the tinted card does not. UNKNOWN which component paints it: a crop of x 1140-1440 plus that component's CSS would settle whether it is --color-green-bg, --color-brand-highlight or a brand gradient | OPEN | P1 |
| 360 | shadow — --shadow-inset-lg | `packages/assets/styles/variables.scss:127`<br>`packages/assets/styles/variables.scss:358` | inset 0px -2px 2px hsla(221,39%,91%,0.1) packages/assets/styles/variables.scss:127; inset 0px -2px 2px hsla(221,39%,11%,0.1) packages/assets/styles/variables.scss:358 | static inner shadow | absent: no shadow field in theme.rs:47-441 | not carried; egui has no inset-shadow primitive, so a faithful port is a banded fill, which shell.rs:531-545 already does by hand for the content surface ⇒ renderer risk: inset shadow, for which egui has no primitive (the report calls for a banded fill) | RENDERER-RISK | P2 |
| 361 | shadow — --shadow-inset | `packages/assets/styles/variables.scss:128`<br>`packages/assets/styles/variables.scss:359` | inset 0px -2px 2px hsla(221,39%,91%,0.05) packages/assets/styles/variables.scss:128; hsla(221,39%,11%,0.05) packages/assets/styles/variables.scss:359 | static | absent | not carried ⇒ renderer risk: inset shadow, for which egui has no primitive | RENDERER-RISK | P2 |
| 362 | shadow — --shadow-inset-sm | `packages/assets/styles/variables.scss:129`<br>`packages/assets/styles/variables.scss:360` | inset 0px -1px 2px hsla(221,39%,91%,0.15) packages/assets/styles/variables.scss:129; inset 0px -1px 1px hsla(221,39%,11%,0.25) packages/assets/styles/variables.scss:360 | static; the well behind a select (packages/ui/src/components/base/DropdownSelect.vue:237, :280) and a settings field (apps/app-frontend/src/features/instances/settings-modal/general-settings.vue:34) | absent | not carried; four call sites in the reference ⇒ renderer risk: inset shadow, for which egui has no primitive | RENDERER-RISK | P2 |
| 363 | shadow — --shadow-raised-lg | `packages/assets/styles/variables.scss:131` | 0px 2px 4px hsla(221,39%,11%,0.2) packages/assets/styles/variables.scss:131, :362 | static | absent | not carried; no call site in the app either | OPEN | P2 |
| 364 | shadow — --shadow-raised | `packages/assets/styles/variables.scss:132-135`<br>`packages/assets/styles/variables.scss:363` | light three stacked hsl(var(--shadow-color) / .15, .12, .09) layers packages/assets/styles/variables.scss:132-135; dark 0px -2px 4px hsla(221,39%,11%,0.1) packages/assets/styles/variables.scss:363 | static | absent | not carried; --shadow-color is referenced at packages/assets/styles/variables.scss:133-135 and defined nowhere in the tree, so the light value is invalid at computed-value time. UNKNOWN what a browser draws for it: a getComputedStyle read of box-shadow would settle it ⇒ renderer risk: multi-layer shadow — three stacked hsl layers | RENDERER-RISK | P2 |
| 365 | shadow — --shadow-floating | `packages/assets/styles/variables.scss:136-138`<br>`packages/assets/styles/variables.scss:364-366` | hsla(0,0%,0%,0.1) 0px 4px 6px -1px, hsla(0,0%,0%,0.1) 0px 2px 4px -1px packages/assets/styles/variables.scss:136-138; dark with rgba(0,0,0,0.06) packages/assets/styles/variables.scss:364-366 | static; popovers and floating charts | absent | not carried; our popovers paint no shadow at all ⇒ renderer risk: multi-layer shadow — two blurred layers on every popover | RENDERER-RISK | P1 |
| 366 | shadow — --shadow-card | `packages/assets/styles/variables.scss:140`<br>`packages/assets/styles/variables.scss:368` | rgba(50,50,100,0.1) 0px 2px 4px 0px packages/assets/styles/variables.scss:140; rgba(0,0,0,0.25) 0px 2px 4px 0px packages/assets/styles/variables.scss:368 | static; every card (apps/app-frontend/src/assets/stylesheets/global.scss:165, Avatar.vue:301, packages/ui/src/components/base/OptionGroup.vue:130) | absent | not carried: cards are flat fills with a 1 px stroke, so they have no elevation at rest ⇒ renderer risk: soft blurred shadow (0 2px 4px) on every card at rest | RENDERER-RISK | P0 |
| 367 | shadow — --shadow-button | `packages/assets/styles/variables.scss:220` | 0 1px 3px 0 rgba(0,0,0,0.05), 0 1px 2px 0 rgba(0,0,0,0.15) packages/assets/styles/variables.scss:220 | static | absent | not carried; used by the accounts-card buttons (apps/app-frontend/src/features/minecraft-accounts/accounts-card.vue:66, :76) and packages/ui/src/components/skin/CapeLikeTextButton.vue:28 ⇒ renderer risk: multi-layer shadow — two layers on button faces | RENDERER-RISK | P2 |
| 368 | shadow — the base button's own edge | `packages/ui/src/components/base/buttons/ButtonFrame.vue:137-139`<br>`packages/ui/src/components/base/buttons/ButtonFrame.vue:16` | box-shadow: inset 0 0 0 1px var(--surface-5), 0 1px 1px rgba(0,0,0,0.12) (packages/ui/src/components/base/buttons/ButtonFrame.vue:137-139), with border-0 at packages/ui/src/components/base/buttons/ButtonFrame.vue:16 | static edge of every base button and of the kbd keycap | home.rs:565-570 fills BUTTON_BG then strokes BUTTON_BORDER, while its own comment at home.rs:563-564 says surface-5 | the ring is drawn in the wrong token and the 0 1px 1px drop layer is missing; in light mode #dddddd against a 0.35 grey composite is a visible difference | OPEN | P1 |
| 369 | splash — --splash-tint-top / --splash-tint-bottom | `packages/assets/styles/variables.scss:163-164`<br>`packages/assets/styles/variables.scss:383-384` | rgba(185,255,207,0.465) / rgba(183,226,255,0.563) packages/assets/styles/variables.scss:163-164; rgba(66,131,92,0.275) / rgba(17,35,43,0.5) packages/assets/styles/variables.scss:383-384 | static; the splash's 180deg stack (apps/app-frontend/src/components/ui/SplashScreen.vue:204) | absent | not carried; no splash screen is drawn | OPEN | P2 |
| 370 | splash — --splash-overlay | `packages/assets/styles/variables.scss:165`<br>`packages/assets/styles/variables.scss:385` | rgba(181,255,213,0.315) packages/assets/styles/variables.scss:165; rgba(22,24,28,0.64) packages/assets/styles/variables.scss:385 | static (apps/app-frontend/src/components/ui/SplashScreen.vue:205) | absent | not carried | OPEN | P2 |
| 371 | splash — --splash-cube-opacity / --splash-cube-blend | `packages/assets/styles/variables.scss:166-167`<br>`packages/assets/styles/variables.scss:386-387` | 0.4 / multiply packages/assets/styles/variables.scss:166-167; 0.8 / normal packages/assets/styles/variables.scss:386-387 | static (apps/app-frontend/src/components/ui/SplashScreen.vue:228-229); egui has no mix-blend-mode | absent | not carried; multiply is UNKNOWN-reproducible in egui without a shader | OPEN | P2 |
| 372 | ad — --color-ad | `packages/assets/styles/variables.scss:211`<br>`packages/assets/styles/variables.scss:230-408`<br>`packages/assets/styles/variables.scss:420` | rgba(125,75,162,0.2) packages/assets/styles/variables.scss:211, declared in the html block so dark inherits it (the dark block at packages/assets/styles/variables.scss:230-408 does not restate it); oled #0d1828 packages/assets/styles/variables.scss:420 | static ad-slot ground | absent | not carried; the sidebar's ad slot is the hardcoded hosting banner at panel.rs:203-220 ⇒ blocked on the ads/promotions service — the ground exists for the ad slot only | BLOCKED-SERVICE | P1 |
| 373 | ad — --color-ad-raised / -contrast / -highlight | `packages/assets/styles/variables.scss:212-214` | rgba(190,140,243,0.5) / black / var(--color-purple) packages/assets/styles/variables.scss:212-214 | static | absent | not carried ⇒ blocked on the ads/promotions service — the inks exist for the ad slot only | BLOCKED-SERVICE | P2 |
| 374 | platform — the 18 --color-platform-* tokens | `packages/assets/styles/variables.scss:169-185`<br>`packages/assets/styles/variables.scss:389-405` | light #8a7b71 .. #dd5088 packages/assets/styles/variables.scss:169-185; dark #dbb69b .. #f45e9a packages/assets/styles/variables.scss:389-405 | static badge inks; no state | theme.rs:334-430 (light theme.rs:741-773, dark theme.rs:568-600) | all 18 carried and all 36 values match; none is read and every one is marked dead_code | OPEN | P2 |
| 375 | radius — --radius-xs | `packages/assets/styles/variables.scss:201` | 0.25rem = 4px packages/assets/styles/variables.scss:201 | static | theme.rs:788 | carried and matches | DONE | P2 |
| 376 | radius — --radius-sm | `packages/assets/styles/variables.scss:202` | 0.5rem = 8px packages/assets/styles/variables.scss:202 | static; the world row (home.rs:143) | theme.rs:791 | carried and matches | DONE | P2 |
| 377 | radius — --radius-md | `packages/assets/styles/variables.scss:203` | 0.75rem = 12px packages/assets/styles/variables.scss:203 | static; the radius of .btn (packages/assets/styles/classes.scss:398) and of an input | theme.rs:794 | carried and matches; the most used radius (instance.rs:32) | DONE | P0 |
| 378 | radius — --radius-lg | `packages/assets/styles/variables.scss:204` | 1rem = 16px packages/assets/styles/variables.scss:204 | static; the window's corner radius (theme.rs:1027) | theme.rs:797 | carried and matches | DONE | P2 |
| 379 | radius — --radius-xl | `packages/assets/styles/variables.scss:205` | 1.25rem = 20px packages/assets/styles/variables.scss:205 | static; the content surface's top-left corner (geometry.rs:11-18 quotes App.vue) | theme.rs:800, with a second copy as geometry.rs:78 | carried and matches, but duplicated: one token, two constants, so the pair can drift | DONE | P1 |
| 380 | radius — --radius-max | `packages/assets/styles/variables.scss:206` | 999999999px packages/assets/styles/variables.scss:206 | static; a pill | theme.rs:806 | carried as 255, the largest egui CornerRadius holds; the sheet's value is clamped rather than carried, which is invisible on a pill | DONE | P2 |
| 381 | radius — rounded-xl and --radius-xl are different ladders | `packages/ui/src/components/base/buttons/ButtonFrame.vue:31`<br>`packages/assets/styles/variables.scss:205` | packages/ui/src/components/base/buttons/ButtonFrame.vue:31 gives a 36 px button rounded-xl, i.e. 12px, while packages/assets/styles/variables.scss:205 says --radius-xl is 20px | static | theme.rs:794 is used for rounded-xl (pages/settings.rs:1036-1037) and theme.rs:800 for --radius-xl (pages/settings.rs:915-916) | both call sites are right, but nothing in the crate records that the two xl values are 12 and 20: only a comment at each site | DONE | P2 |
| 382 | gap — --gap-xs | `packages/assets/styles/variables.scss:195` | 0.25rem = 4px packages/assets/styles/variables.scss:195 | static | theme.rs:809 | carried and matches, never read (no call site outside theme.rs) | DONE | P2 |
| 383 | gap — --gap-sm | `packages/assets/styles/variables.scss:196` | 0.5rem = 8px packages/assets/styles/variables.scss:196 | static | theme.rs:812 | carried and matches, never read | DONE | P2 |
| 384 | gap — --gap-md | `packages/assets/styles/variables.scss:197` | 0.75rem = 12px packages/assets/styles/variables.scss:197 | static; cited by name at pages/skins.rs:349 | theme.rs:815 | carried and matches, never read: every page writes its own 12.0 | DONE | P2 |
| 385 | gap — --gap-lg | `packages/assets/styles/variables.scss:198` | 1rem = 16px packages/assets/styles/variables.scss:198 | static | theme.rs:818 | carried and matches, never read | DONE | P2 |
| 386 | gap — --gap-xl | `packages/assets/styles/variables.scss:199` | 1.5rem = 24px packages/assets/styles/variables.scss:199 | static | theme.rs:821 | carried and matches, never read; Theme::gaps() (theme.rs:1096) has no caller | DONE | P2 |
| 387 | scheme — dark is the default | `app-frontend/src/assets/stylesheets/apps/app-frontend/src/assets/stylesheets/global.scss:45`<br>`packages/assets/styles/variables.scss:225-228` | :root { color-scheme: dark } (app-frontend/src/assets/stylesheets/apps/app-frontend/src/assets/stylesheets/global.scss:45); .light-mode and .oled-mode are user choices (packages/assets/styles/variables.scss:225-228 and :410-434) | a switch in Settings swaps the whole table; the reference cross-fades colours over 125ms | shell.rs:414-421 (ThemeChoice Dark, Light, Oled) | reachable, so every light and oled row above is a real screen rather than a hypothetical | DONE | P1 |
| 388 | scheme — the OLED surface table | `packages/assets/styles/variables.scss:412-418` | --surface-1..5 = #000000, #050506, #09090a, #0c0d11, #101013, #1b1b20, #25262b (packages/assets/styles/variables.scss:412-418) | static | theme.rs:608-616 sets BG #000000, RAISED_BG #080808, SURFACE_1 #0d0d0d and SUPER_RAISED_BG #121212, then inherits the rest of DARK | 8 of the 10 oled-visible values are wrong (#080808, #0d0d0d and #121212 appear nowhere in the sheet) and the oled gradient overrides at packages/assets/styles/variables.scss:422-433 are absent | OPEN | P1 |

## 12. Typography

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 389 | type-size — --font-size-xxs | `packages/assets/styles/defaults.scss:23` | 0.625rem = 10px packages/assets/styles/defaults.scss:23 | static | absent | not carried; the smallest size the sheet declares | OPEN | P2 |
| 390 | type-size — --font-size-xs | `packages/assets/styles/defaults.scss:24` | 0.75rem = 12px packages/assets/styles/defaults.scss:24 | static; text-xs, 20 uses, e.g. the kbd keycap at apps/app-frontend/src/components/ui/WelcomeScreen.vue:118 | theme.rs:839 | carried and matches, and is one of only two sizes used by name (home.rs:892, :967); the keycap itself writes 12.0 by hand (home.rs:515) | OPEN | P1 |
| 391 | type-size — --font-size-sm | `packages/assets/styles/defaults.scss:25` | 0.875rem = 14px packages/assets/styles/defaults.scss:25 | static; text-sm, 86 uses, e.g. the instance version line at apps/app-frontend/src/components/ui/library/instance-group/instance-card-view.vue:87 | absent as a constant | not carried: 20 sites write 13.0 (pages/library.rs:645, pages/home.rs:721, pages/project.rs:135) where the ladder step is 14, so that text is set 1 px small | OPEN | P0 |
| 392 | type-size — --font-size-nm | `packages/assets/styles/defaults.scss:26` | 1rem = 16px packages/assets/styles/defaults.scss:26 | static; text-base, 62 uses, e.g. the instance name at apps/app-frontend/src/components/ui/library/instance-group/instance-card-view.vue:80 and the welcome description at apps/app-frontend/src/components/ui/WelcomeScreen.vue:98 | theme.rs:842 | carried and matches; used at 2 sites (home.rs:892, :967); other 16 px text is a literal (e.g. home.rs:446) | DONE | P1 |
| 393 | type-size — --font-size-md | `packages/assets/styles/defaults.scss:27` | 1.125rem = 18px packages/assets/styles/defaults.scss:27 | static; text-lg, 17 uses, e.g. packages/ui/src/components/base/SettingsLabel.vue:35 | absent as a constant | not carried: 3 sites write 17.0 (pages/home.rs:709, shell.rs:1691) where the step is 18, so one class renders at two sizes across pages | OPEN | P0 |
| 394 | type-size — --font-size-lg | `packages/assets/styles/defaults.scss:28` | 1.25rem = 20px packages/assets/styles/defaults.scss:28 | static; text-xl, 5 uses | absent as a constant | not carried; 20.0 is written at 4 sites (about.rs:228, project.rs:130, servers.rs:538, skins.rs:216) | OPEN | P2 |
| 395 | type-size — --font-size-xl | `packages/assets/styles/defaults.scss:29` | 1.5rem = 24px packages/assets/styles/defaults.scss:29 | static; text-2xl, 11 uses, e.g. the Library heading apps/app-frontend/src/components/ui/library/index.vue:268 and the welcome title apps/app-frontend/src/components/ui/WelcomeScreen.vue:95 | theme.rs:845 | carried and matches; 24.0 is also written directly at home.rs:438, library.rs:338 and library.rs:714 | OPEN | P0 |
| 396 | type-size — --font-size-2xl | `packages/assets/styles/defaults.scss:30` | 2rem = 32px packages/assets/styles/defaults.scss:30 | static; used as a min-height at packages/assets/styles/classes.scss:502, and it is not Tailwind's text-2xl | absent | not carried; the name collides with the Tailwind class, whose 24px is what text-2xl means at every call site | OPEN | P2 |
| 397 | type-size — --font-size-3xl | `packages/assets/styles/defaults.scss:31` | 3rem = 48px packages/assets/styles/defaults.scss:31 | static | absent | not carried; our largest heading is 30.0 (servers.rs:623), which is Tailwind's text-3xl rather than this token, and neither value is stated in this tree | OPEN | P2 |
| 398 | type-weight — --font-weight-regular | `packages/assets/styles/defaults.scss:34` | 400 packages/assets/styles/defaults.scss:34 | static; body's own weight (packages/assets/styles/defaults.scss:18) | theme.rs:848 | carried and matches; fonts.rs body() is the 400 face (fonts.rs:81) | DONE | P0 |
| 399 | type-weight — --font-weight-medium | `packages/assets/styles/defaults.scss:35` | 500 packages/assets/styles/defaults.scss:35 | static; font-medium, 72 uses, and every .button-base (packages/assets/styles/classes.scss:206) | theme.rs:851, as --font-weight-text | carried and matches; fonts.rs label() is 500 (fonts.rs:86) | DONE | P0 |
| 400 | type-weight — --font-weight-bold | `packages/assets/styles/defaults.scss:36` | 700 packages/assets/styles/defaults.scss:36 | static; font-bold, 22 uses | absent as a token | not carried as a token even though fonts.rs title() ships the 700 face (fonts.rs:96); type_weights() at theme.rs:1116 returns only 400, 500 and 800 | OPEN | P2 |
| 401 | type-weight — --font-weight-extrabold | `packages/assets/styles/defaults.scss:37` | 800 packages/assets/styles/defaults.scss:37 | static; font-extrabold, 11 uses | theme.rs:854, as --font-weight-heading | carried and matches; fonts.rs heading() is 800 (fonts.rs:101) | DONE | P2 |
| 402 | type-weight — --font-weight-text | `packages/assets/styles/defaults.scss:39` | var(--font-weight-medium) packages/assets/styles/defaults.scss:39 | static; the whole interface's label weight | theme.rs:851 | carried and matches | DONE | P0 |
| 403 | type-weight — --font-weight-heading | `packages/assets/styles/defaults.scss:40` | var(--font-weight-extrabold) packages/assets/styles/defaults.scss:40 | static, but the app's headings do not use it: .heading-2xl is text-2xl font-semibold (packages/ui/src/styles/tailwind-utilities.css:110-112) and the Library heading is text-2xl font-semibold (apps/app-frontend/src/components/ui/library/index.vue:268) | theme.rs:854 | carried and matches the sheet yet is the wrong weight for the element that looks like a heading: those headings are 600, which is fonts.rs semibold (fonts.rs:91) and not a token this table carries | OPEN | P0 |
| 404 | type-weight — --font-weight-title | `packages/assets/styles/defaults.scss:41` | var(--font-weight-extrabold) packages/assets/styles/defaults.scss:41 | static; declared and never read in the tree | absent | not carried; unused in the reference too | NOT-IN-SOURCE | P2 |
| 405 | type-weight — font-semibold | `packages/assets/styles/defaults.scss:34-37`<br>`packages/assets/styles/inter.scss:19-27`<br>`packages/ui/src/components/base/buttons/ButtonFrame.vue:31`<br>`packages/ui/src/components/base/page-header/apps/app-frontend/src/components/ui/library/index.vue:13` | 600 is not declared in packages/assets/styles/defaults.scss:34-37, but the 600 face ships at packages/assets/styles/inter.scss:19-27 and the class is used 121 times, more than any other weight (packages/ui/src/components/base/buttons/ButtonFrame.vue:31, packages/ui/src/components/base/page-header/apps/app-frontend/src/components/ui/library/index.vue:13) | static; button labels and headings | fonts.rs:29 (inter-600), fonts.rs:91 (semibold()) | exposed by fonts.rs and absent from the token table: type_weights() at theme.rs:1116 returns 400, 500 and 800, so nothing in the crate says 600 exists | OPEN | P0 |
| 406 | font-family — --font-standard | `packages/assets/styles/defaults.scss:12-14`<br>`packages/assets/styles/defaults.scss:16`<br>`packages/assets/styles/defaults.scss:17`<br>`packages/assets/styles/defaults.scss:18` | Inter, -apple-system, BlinkMacSystemFont, Segoe UI, Oxygen, Ubuntu, Roboto, Cantarell, Fira Sans, Droid Sans, Helvetica Neue, sans-serif (packages/assets/styles/defaults.scss:12-14), applied at packages/assets/styles/defaults.scss:16 with font-size 16px at packages/assets/styles/defaults.scss:17 and font-weight var(--font-weight-regular) at packages/assets/styles/defaults.scss:18 | static | fonts.rs:26-32 (Inter 400, 500, 600, 700, 800 embedded) and fonts.rs:60-67 (400 as the default family) | the family matches: five Inter weights, the same five packages/assets/styles/inter.scss:4-45 declares; the fallback chain is egui's rather than the sheet's | DONE | P2 |
| 407 | font-family — --mono-font | `packages/assets/styles/defaults.scss:15` | ui-monospace, SFMono-Regular, SF Mono, Menlo, Consolas, Liberation Mono, monospace (packages/assets/styles/defaults.scss:15) | static; the font-mono class is used 18 times in packages/ui, e.g. packages/ui/src/components/base/CopyCode.vue:3 | absent: fonts.rs:38-70 registers no mono family | not carried; monospace text falls back to egui's proportional face | OPEN | P2 |
| 408 | type-usage — text-xs | `apps/app-frontend/src/components/ui/WelcomeScreen.vue:118`<br>`packages/ui/src/components/base/NavTabs.vue:9`<br>`packages/ui/src/components/base/CopyCode.vue:3` | 20 uses, e.g. apps/app-frontend/src/components/ui/WelcomeScreen.vue:118, packages/ui/src/components/base/NavTabs.vue:9, packages/ui/src/components/base/CopyCode.vue:3 | static | fonts.rs:81-103 take a bare f32, so the size belongs to the call site | no size token is passed: the keycap is body(12.0) at home.rs:515, right by hand and unenforced | OPEN | P1 |
| 409 | type-usage — text-sm | `apps/app-frontend/src/components/ui/library/instance-group/instance-card-view.vue:87`<br>`apps/app-frontend/src/components/ui/WelcomeScreen.vue:114`<br>`apps/app-frontend/src/app/shell/title-bar/action-bar.vue:17` | 86 uses, e.g. apps/app-frontend/src/components/ui/library/instance-group/instance-card-view.vue:87, apps/app-frontend/src/components/ui/WelcomeScreen.vue:114, apps/app-frontend/src/app/shell/title-bar/action-bar.vue:17 | static | fonts.rs:81-103 | 13.0 at 20 sites and 14.0 at 7: one ladder step is rendered at two sizes | OPEN | P0 |
| 410 | type-usage — text-base | `apps/app-frontend/src/components/ui/library/instance-group/instance-card-view.vue:80`<br>`apps/app-frontend/src/components/ui/WelcomeScreen.vue:98`<br>`apps/app-frontend/src/app/shell/sidebar/index.vue:112` | 62 uses, e.g. apps/app-frontend/src/components/ui/library/instance-group/instance-card-view.vue:80, apps/app-frontend/src/components/ui/WelcomeScreen.vue:98, apps/app-frontend/src/app/shell/sidebar/index.vue:112 | static | fonts.rs:81-103 | 16.0 at 2 sites, 15.0 at 4 (project.rs:411, :478, :538; settings.rs:2215) and 17.0 at 3 | OPEN | P0 |
| 411 | type-usage — text-lg | `packages/ui/src/components/base/SettingsLabel.vue:35`<br>`packages/ui/src/components/base/Admonition.vue:31`<br>`packages/ui/src/components/base/ErrorInformationCard.vue:13` | 17 uses, e.g. packages/ui/src/components/base/SettingsLabel.vue:35, packages/ui/src/components/base/Admonition.vue:31, packages/ui/src/components/base/ErrorInformationCard.vue:13 | static | fonts.rs:81-103 | 17.0 at 3 sites and 18.0 at 1; neither is checked against a token | OPEN | P1 |
| 412 | type-usage — text-xl | `packages/ui/src/components/base/MultiStageModal.vue:58`<br>`apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:388` | 5 uses, e.g. packages/ui/src/components/base/MultiStageModal.vue:58, apps/app-frontend/src/components/ui/skin/VirtualSkinSectionList.vue:388 | static | fonts.rs:81-103 | 20.0 at 4 sites | OPEN | P2 |
| 413 | type-usage — text-2xl | `packages/ui/src/components/base/page-header/apps/app-frontend/src/components/ui/library/index.vue:13`<br>`apps/app-frontend/src/components/ui/library/index.vue:268`<br>`packages/ui/src/components/base/EmptyState.vue:7`<br>`apps/app-frontend/src/components/ui/WelcomeScreen.vue:95` | 11 uses, e.g. packages/ui/src/components/base/page-header/apps/app-frontend/src/components/ui/library/index.vue:13, apps/app-frontend/src/components/ui/library/index.vue:268, packages/ui/src/components/base/EmptyState.vue:7, apps/app-frontend/src/components/ui/WelcomeScreen.vue:95 | static, and always with font-semibold | fonts.rs:91 at 24.0 (home.rs:438, library.rs:338) | matches | DONE | P0 |
| 414 | type-usage — text-3xl and text-4xl | `apps/app-frontend/src/components/ui/HostingUpdateRequired.vue:94`<br>`packages/ui/src/styles/tailwind-utilities.css:116` | text-3xl 1 use (apps/app-frontend/src/components/ui/HostingUpdateRequired.vue:94); text-4xl 1 use (packages/ui/src/styles/tailwind-utilities.css:116) | static | theme.rs:839-845 has no size above 24 | no token; pages/servers.rs:623 writes 30.0 for an empty-state heading, a value the sheet does not state for text-3xl | OPEN | P2 |
| 415 | type-usage — font-normal, -medium, -semibold, -bold, -extrabold | — (the report gives no file:line) | 12, 72, 121, 22 and 11 uses across the scanned files | static | fonts.rs:81, fonts.rs:86, fonts.rs:91, fonts.rs:96, fonts.rs:101 | all five weights are exposed and each maps to the right face; the token table in theme.rs lists neither 600 nor 700 | OPEN | P1 |

## 13. Motion

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 416 | motion — floating-expand | `packages/ui/src/styles/tailwind-utilities.css:56-74` | opacity 0.125s var(--ease-out-expo) and scale 0.125s var(--ease-out-expo), with transform-origin top (packages/ui/src/styles/tailwind-utilities.css:56-74) | enters from opacity 0 and scale 0.9 0.3, leaves in reverse | theme.rs:923 (POPOVER_MS) and theme.rs:886-889 (EASE_OUT_EXPO) | the numbers are transcribed and never applied: the kebab menu (home.rs:932) appears with no scale and no fade | OPEN | P1 |
| 417 | motion — the easing constants | `packages/ui/src/styles/tailwind-utilities.css:2`<br>`packages/ui/src/styles/tailwind-utilities.css:42` | --ease-out-expo cubic-bezier(0.16,1,0.3,1) (packages/ui/src/styles/tailwind-utilities.css:2); the floating bar's cubic-bezier(0.15,1.4,0.64,0.96) (packages/ui/src/styles/tailwind-utilities.css:42) | static definitions used by the transitions above | theme.rs:886-903 | all three curves are carried and correct, and none is used | OPEN | P2 |
| 418 | tokens — `--ease-out-expo` | `packages/ui/src/styles/tailwind-utilities.css:2` | `packages/ui/src/styles/tailwind-utilities.css:2` = cubic-bezier(0.16, 1, 0.3, 1) | named easing only, no duration of its own; consumed by the NavButton plate, floating-expand, tooltip-moving and CollapsibleRegion | `crates/sift-ui/src/theme.rs:885-889` | constant transcribed correctly but no drawing code reads it; `Theme::easing` has no caller outside `theme.rs:1391-1392` | OPEN | P1 |
| 419 | tokens — `ease-in-out` (CSS spec curve) | `packages/assets/styles/classes.scss:193-197`<br>`packages/assets/styles/defaults.scss:98-102` | `packages/assets/styles/classes.scss:193-197` and `packages/assets/styles/defaults.scss:98-102` name it | [0.42, 0, 0.58, 1]; the curve on the clickable, toast, modal body and loading bar | `crates/sift-ui/src/theme.rs:878-883` | declared and never used to tween anything | OPEN | P1 |
| 420 | tokens — floating overshoot bezier | `packages/ui/src/styles/tailwind-utilities.css:42`<br>`apps/app-frontend/src/App.vue:2375` | `packages/ui/src/styles/tailwind-utilities.css:42` and `apps/app-frontend/src/App.vue:2375` spell out cubic-bezier(0.15, 1.4, 0.64, 0.96) | 250ms on the floating action bar, 500ms on nav-button enter | `crates/sift-ui/src/theme.rs:891-896` | constant only; neither element exists yet | OPEN | P2 |
| 421 | tokens — duration ladder | `packages/assets/styles/classes.scss:193-197`<br>`packages/ui/src/styles/tailwind-utilities.css:70-72`<br>`packages/ui/src/components/modal/NewModal.vue:668`<br>`packages/ui/src/components/notifications/NotificationStack.vue:53`<br>`packages/ui/src/components/base/Toggle.vue:8` | `packages/assets/styles/classes.scss:193-197` (0.5s opacity, 0.2s filter, 0.05s scale, 0.2s outline), `packages/ui/src/styles/tailwind-utilities.css:70-72` (125ms), `packages/ui/src/components/modal/NewModal.vue:668` (0.2s), `packages/ui/src/components/notifications/NotificationStack.vue:53` (0.3s), `packages/ui/src/components/base/Toggle.vue:8` (200ms) | 50 / 80 / 100 / 125 / 150 / 200 / 250 / 300 / 500 / 900 / 2000 / 4000ms across the tree | `crates/sift-ui/src/theme.rs:907-927` and `:944-981` | the table has six rows only: the 300ms toast, 250ms toggle, 200ms ready-fade and 80ms tab durations are not in it | OPEN | P1 |
| 422 | tokens — `Theme::motion()` / `Theme::easing()` consumers | — (the report gives no file:line) | not stated in source (the reference applies these in CSS, it has no table) | n/a | `crates/sift-ui/src/theme.rs:1120-1128` | both accessors have zero call sites outside the test at `theme.rs:1366-1393`; MOTION documents motion, nothing animates | OPEN | P0 |
| 423 | system — theme colour switch | `packages/ui/src/styles/tailwind-utilities.css:5-25` | `packages/ui/src/styles/tailwind-utilities.css:5-25` colour, background-color, border-color, outline-color, fill, stroke, box-shadow and caret-color 125ms ease-in-out, gated on `html.theme-color-transitioning` ‖ tokens.md row 111 — element "motion — theme colour transition": color, background-color, border-color, outline-color, fill, stroke and box-shadow over 125ms ease-in-out while html.theme-color-transitioning is set (packages/ui/src/styles/tailwind-utilities.css:5-25) | every colour cross-fades 125ms on one class toggle, with animations suppressed for the duration ‖ tokens.md row 111: triggered by the theme switch, and not exempted by reduced motion | `crates/sift-ui/src/shell.rs:411-426` ‖ tokens.md row 111: shell.rs:411-426 swaps the whole Theme and asks for one repaint | new `Theme` is assigned and one repaint is requested; colours cut over in a single frame ‖ tokens.md row 111: the switch is instant: every colour jumps in one frame instead of 125ms | OPEN | P1 |
| 424 | system — `prefers-reduced-motion` | `packages/assets/styles/defaults.scss:116-121`<br>`packages/ui/src/components/modal/NewModal.vue:690-692`<br>`apps/app-frontend/src/App.vue:2373` | `packages/assets/styles/defaults.scss:116-121` (transform none on `.button-animation` and every button), `packages/ui/src/components/modal/NewModal.vue:690-692` and `:754-756` (transition none), `apps/app-frontend/src/App.vue:2373` (nav enter guarded by no-preference) ‖ tokens.md row 112 — element "motion — prefers-reduced-motion": transform: none !important for .button-animation and button (packages/assets/styles/defaults.scss:116-121) | OS preference removes press scale and modal motion ‖ tokens.md row 112: suppresses the press scale entirely | none in `crates/sift-ui/src` (no reduced-motion flag in `crates/sift-ui/src`) ‖ tokens.md row 112: absent | no equivalent; egui has no media query, so this needs an explicit setting ‖ tokens.md row 112: no reduced-motion branch exists; egui has no such query, so a settings flag would be the equivalent | OPEN | P2 |
| 425 | shell — rail active plate (NavButton `::before`) | `apps/app-frontend/src/components/ui/NavButton.vue:59-71` | `apps/app-frontend/src/components/ui/NavButton.vue:59-71` (rest: opacity 0, scale 0.4, `--color-button-bg-selected`, radius 50%), `:81-88` (selected: scale 1, opacity 1) | opacity 0.25s var(--ease-out-expo) and scale 0.25s var(--ease-out-expo); trigger = the route becomes active | `crates/sift-ui/src/shell.rs:747-753` | plate is a full-size `circle_filled` on the press frame; no 0.4 to 1 growth, no fade | OPEN | P0 |
| 426 | shell — rail hover plate and ink | `apps/app-frontend/src/components/ui/NavButton.vue:13` | `apps/app-frontend/src/components/ui/NavButton.vue:13` (`hover:bg-button-bg hover:text-contrast`, plus `transition-all`, duration not stated in source) | plate and label colour swap on hover; the transition-all duration is a Tailwind default not stated in source | `crates/sift-ui/src/shell.rs:751-760` | plate and ink snap on the hover frame; no ramp | OPEN | P1 |
| 427 | shell — active rail icon drop-shadow | `apps/app-frontend/src/components/ui/NavButton.vue:74-79` | `apps/app-frontend/src/components/ui/NavButton.vue:74-79` `filter: drop-shadow(0 0 0.5rem black)` under active and subpage-active | static filter, no transition | `crates/sift-ui/src/shell.rs:761-766` | icon is drawn in `BUTTON_TEXT_SELECTED` with no drop shadow | OPEN | P1 |
| 428 | shell — rail subpage-active state | `apps/app-frontend/src/components/ui/NavButton.vue:90-92`<br>`apps/app-frontend/src/App.vue:2017-2020` | `apps/app-frontend/src/components/ui/NavButton.vue:90-92` plus `apps/app-frontend/src/App.vue:2017-2020` and `:2028-2030` (Discover is subpage-active while a project route is open) | plate becomes `bg-button-bg` (not selected) and ink `text-contrast` for the parent destination | `crates/sift-ui/src/shell.rs:747` | only `destination == self.rail` selects; an open project leaves Discover unplated | OPEN | P2 |
| 429 | shell — rail create-button pop-in (nav-button-animated) | `apps/app-frontend/src/App.vue:2373-2417` | `apps/app-frontend/src/App.vue:2373-2417` | enter all 0.5s cubic-bezier(0.15, 1.4, 0.64, 0.96) from scale 0.5, translate -2rem, opacity 0; leave all 0.25s ease to scale 0.75, opacity 0; `::before` pop 0.5s ease-in, scale 0.5 to 1.5 with opacity 0 to 0.5; the whole block sits behind no-preference | `crates/sift-ui/src/shell.rs:769-803` | create button is drawn unconditionally; no enter, no leave, no pop ring | OPEN | P2 |
| 430 | shell — rail tooltip pill | `packages/ui/src/providers/tooltip.ts:35-36` | `packages/ui/src/providers/tooltip.ts:35-36` SHOW_DELAY 300ms and HIDE_DELAY 400ms | hover-triggered; 300ms dwell before show, 400ms grace before hide | `crates/sift-ui/src/shell.rs:438-460` | pill mounts on the hovered frame and unmounts on leave; neither delay exists | OPEN | P1 |
| 431 | shell — window control hover | `apps/app-frontend/src/app/shell/title-bar/window-controls.vue:29` | `apps/app-frontend/src/app/shell/title-bar/window-controls.vue:29` (`hover:!bg-red` on close, `before:` plate); duration not stated in source | hover only, no transition declared | `crates/sift-ui/src/shell.rs:677-686` | no gap on motion: both switch on the hover frame | DONE | P2 |
| 432 | shell — sidebar show/hide | `apps/app-frontend/src/App.vue:2325` | `apps/app-frontend/src/App.vue:2325` `transition: grid-template-columns 0.4s ease-in-out` is commented out; `:2327-2329` only the column value changes | no animation by design in 0.22.0 | none in `crates/sift-ui/src` (no sidebar toggle in `crates/sift-ui/src/shell.rs`) | no motion gap; the toggle itself is missing | OPEN | P2 |
| 433 | shell — title-bar sidebar toggle rotation | `apps/app-frontend/src/app/shell/title-bar/index.vue:70-71` | `apps/app-frontend/src/app/shell/title-bar/index.vue:70-71` `transition-transform` with `rotate-180` when untoggled | rotation on toggle; duration not stated in source | none in `crates/sift-ui/src` (control not drawn) | absent | OPEN | P2 |
| 434 | shell — toast | `packages/ui/src/components/notifications/NotificationStack.vue:50-60` | `packages/ui/src/components/notifications/NotificationStack.vue:50-60` enter, leave and move all 0.3s ease-in-out, from `translateX(100%) scale(0.95)` and opacity 0 | 300ms slide in from the right and out to the right; trigger = mount and unmount | `crates/sift-ui/src/shell.rs:466-483` | toast is a static plate while `self.toast` is `Some`; no slide, no fade, no dwell timer | OPEN | P0 |
| 435 | shell — restart overlay | `apps/app-frontend/src/App.vue:1970-1984` | `apps/app-frontend/src/App.vue:1970-1984` (`Transition name="fade"`) and `:2419-2425` fade enter 0.25s ease-in-out from opacity 0 | 250ms cross-fade while restarting | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 436 | shell — page route swap | `apps/app-frontend/src/App.vue:2203-2209` | `apps/app-frontend/src/App.vue:2203-2209` RouterView inside Suspense with no Transition wrapper | no animation: old page unmounts and the new one mounts on one frame | `crates/sift-ui/src/shell.rs:302-390` (page picked by if/else) | none: the reference animates nothing here either | DONE | P2 |
| 437 | modal — modal resize | `packages/ui/src/components/modal/NewModal.vue:338-343` | `packages/ui/src/components/modal/NewModal.vue:338-343` `element.animate(..., { duration: 200, easing: 'ease-in-out' })` | 200ms ease-in-out when the dialog's height changes | none in `crates/sift-ui/src` | absent; heights snap | OPEN | P2 |
| 438 | buttons — ButtonFrame | `packages/ui/src/components/base/buttons/ButtonFrame.vue:18-19`<br>`packages/ui/src/components/base/buttons/ButtonFrame.vue:18-24` | `packages/ui/src/components/base/buttons/ButtonFrame.vue:18-19` transition of background-color, color, box-shadow, filter, opacity and transform at `duration-150 ease-out`, plus `enabled:active:scale-[0.97]`; `:24-25` disabled is opacity 50 ‖ tokens.md row 109 — element "motion — ButtonFrame, the family the app actually uses": transition-[background-color,color,box-shadow,filter,opacity,transform] duration-150 ease-out, enabled:active:scale-[0.97], hover brightness(--hover-brightness), focus-visible ring-4 ring-brand-shadow, disabled opacity-50 (packages/ui/src/components/base/buttons/ButtonFrame.vue:18-24) | 150ms ease-out on hover, focus and press; press shrinks to 0.97 ‖ tokens.md row 109: hover 150ms ease-out; press scale 0.97 over 150ms; focus ring 4 px in --color-brand-shadow | `crates/sift-ui/src/shell.rs:678-692` and `crates/sift-ui/src/pages/settings.rs:2375-2391` ‖ tokens.md row 109: theme.rs:907-919 (press 50ms, hover 200ms) and theme.rs:1052-1067 | no press scale and no 150ms ramp on any control; fills swap in one frame ‖ tokens.md row 109: the 150ms, 0.97 and ring-4 contract is not what theme.rs records (50ms, 0.95, no ring), so a future tween would use the wrong numbers | OPEN | P0 |
| 439 | buttons — legacy clickable press | `packages/assets/styles/defaults.scss:97-106`<br>`packages/assets/styles/classes.scss:191-201`<br>`packages/assets/styles/defaults.scss:98-106`<br>`packages/assets/styles/classes.scss:191-202`<br>`packages/assets/styles/classes.scss:205` | `packages/assets/styles/defaults.scss:97-106` (opacity 0.5s, filter 0.2s, transform 0.05s, outline 0.2s ease-in-out; `:active` transform scale 0.95) and `packages/assets/styles/classes.scss:191-201` (same list with `scale` and `scale: 0.95`) ‖ tokens.md row 106 — element "motion — .button-animation": opacity 0.5s ease-in-out, filter 0.2s ease-in-out, transform 0.05s ease-in-out, outline 0.2s ease-in-out, and :active:not(:disabled) transform: scale(0.95) (packages/assets/styles/defaults.scss:98-106) ‖ tokens.md row 107 — element "motion — .clickable": transition opacity 0.5s, filter 0.2s, scale 0.05s, outline 0.2s, and :active scale 0.95 (packages/assets/styles/classes.scss:191-202); .button-base extends it (packages/assets/styles/classes.scss:205) | press = 50ms to 0.95, release = 50ms back; hover = 200ms filter ‖ tokens.md row 106: hover is the filter over 200ms ease-in-out; press is scale 0.95 over 50ms; release reverses ‖ tokens.md row 107: the older family, still the base of every .btn | `crates/sift-ui/src/theme.rs:907` (PRESS_MS 50) ‖ tokens.md row 106: no tween exists in the crate: the hover fills are settled colours (panel.rs:332-333 says so in words) and the only animation is fastframe's scroll glide (shell.rs:95-96) ‖ tokens.md row 107: theme.rs:944-981 (scale, filter, outline and opacity rows) | constant exists, no control is drawn with a press scale ‖ tokens.md row 106: the four transitions are transcribed into the MOTION table at theme.rs:944-981 but nothing consumes it (no call site) and no press scale is drawn ‖ tokens.md row 107: transcribed, unconsumed, undrawn RE-VERIFIED 2026-11-01: the evaluator now exists — `motion::value` solves a CSS cubic-bezier on egui's clock with three tests (motion.rs:62-172) — and no call site anywhere in the crate reads it, so the press scale is still not drawn. | OPEN-WITH-CAUSE | P1 |
| 440 | buttons — hover/active brightness | `packages/assets/styles/classes.scss:210-221`<br>`packages/assets/styles/classes.scss:210-233` | `packages/assets/styles/classes.scss:210-221` `filter: brightness(0.85)` on hover and focus, `brightness(0.8)` on active, no transition declared ‖ tokens.md row 108 — element "motion — .button-base hover, active and disabled": hover and focus-visible filter brightness(0.85); active brightness(0.8); disabled grayscale(50%) and opacity 0.5 (packages/assets/styles/classes.scss:210-233) | instant filter swap ‖ tokens.md row 108: hover over 200ms ease-in-out on the filter (packages/assets/styles/classes.scss:195) | `crates/sift-ui/src/theme.rs:1087-1093` (`hovered()` multiplies channels, factor 0.9 light and 1.25 dark per `variables.scss:187` and `:407`) ‖ tokens.md row 108: theme.rs:1052-1067 | same factor and same instant swap, so no visual gap; nothing to add ‖ tokens.md row 108: hover and active both use --hover-brightness, which is 1.25 in dark, where this rule darkens to 0.85 and 0.8; no disabled opacity is painted | OPEN | P1 |
| 441 | buttons — outline and transparent buttons | `packages/assets/styles/classes.scss:284-288` | `packages/assets/styles/classes.scss:284-288` background-color, border-color and color 0.2s ease-in-out | 200ms colour ramp on hover | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 442 | buttons — SmartClickable contents | `packages/ui/src/components/base/SmartClickable.vue:42` | `packages/ui/src/components/base/SmartClickable.vue:42` and `:51` `scale 0.125s ease-out`, `:73` opacity 0.125s ease-out, `:86` `scale: var(--active-scale, 0.98)` on `:active` | 125ms scale on press, 125ms opacity dip when brightened | none in `crates/sift-ui/src` | absent (cards are not pressable) | OPEN | P2 |
| 443 | controls — Checkbox | `packages/ui/src/components/base/Checkbox.vue:21` | `packages/ui/src/components/base/Checkbox.vue:21` `group-active:scale-95` on the box; `:9` `hover:brightness-[--hover-brightness]` | press shrinks to 0.95; both states are instant because no transition class is applied | none in `crates/sift-ui/src` (no checkbox drawn) | absent | OPEN | P1 |
| 444 | controls — Chips and filter pills | `packages/ui/src/components/base/Chips.vue:10`<br>`packages/ui/src/components/base/FilterPills.vue:43`<br>`packages/ui/src/components/base/FilterBar.vue:10` | `packages/ui/src/components/base/Chips.vue:10` `.btn` plus `hover:!brightness-125`; `packages/ui/src/components/base/FilterPills.vue:43` `transition-all duration-100 active:scale-[0.97]`; `packages/ui/src/components/base/FilterBar.vue:10` same at `duration-100` | 100ms all-property transition, press shrinks to 0.97 | none in `crates/sift-ui/src` | absent | OPEN | P1 |
| 445 | controls — Tabs, tags and radio buttons | `packages/ui/src/components/base/Tabs.vue:17`<br>`packages/ui/src/components/base/TagItem.vue:4`<br>`packages/ui/src/components/base/RadioButtons.vue:6`<br>`packages/ui/src/components/base/BigOptionButton.vue:3` | `packages/ui/src/components/base/Tabs.vue:17` `transition-all active:scale-[0.97]`; `packages/ui/src/components/base/TagItem.vue:4` `active:scale-[0.95]`; `packages/ui/src/components/base/RadioButtons.vue:6` `active:scale-95`; `packages/ui/src/components/base/BigOptionButton.vue:3` `active:scale-[0.98]` with `hover:brightness-110` and `:23` a 100ms hover reveal | press scale 0.95 to 0.98; duration not stated in source | none in `crates/sift-ui/src` | absent | OPEN | P1 |
| 446 | controls — SegmentedControl indicator | `packages/ui/src/components/base/SegmentedControl.vue:147-153` | `packages/ui/src/components/base/SegmentedControl.vue:147-153` left and right 80ms cubic-bezier(0.4, 0, 0.2, 1) with a 175ms trailing stagger (`:27`, `:60-63`), background-color and box-shadow 100ms ease | 80ms move plus a 175ms delayed trailing edge, side depends on travel direction | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 447 | controls — OptionGroup plate | `packages/ui/src/components/base/OptionGroup.vue:123-127` | `packages/ui/src/components/base/OptionGroup.vue:123-127` `all 150ms cubic-bezier(0.4, 0, 0.2, 1)` and `opacity 250ms cubic-bezier(0.5, 0, 0.2, 1) 50ms` | 150ms move, opacity 250ms starting 50ms in | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 448 | controls — Slider | `packages/ui/src/components/base/Slider.vue:301-310` | `packages/ui/src/components/base/Slider.vue:301-310` width 0.25s var(--ease-out-expo) and clip-path 0.25s var(--ease-out-expo), both `none` under reduced motion | 250ms ease-out-expo on drag and on keyboard step | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 449 | controls — ProgressSpinner | `packages/ui/src/components/base/ProgressSpinner.vue:50` | `packages/ui/src/components/base/ProgressSpinner.vue:50` `transition: --_progress 0.125s ease-in-out` | 125ms smoothing of the arc's custom property | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 450 | controls — input focus ring | `apps/app-frontend/src/assets/stylesheets/global.scss:213`<br>`packages/ui/src/components/base/inputs/InputFrame.vue:4` | `apps/app-frontend/src/assets/stylesheets/global.scss:213` `transition: box-shadow 0.1s ease-in-out` to the ring at `:216-222`; `packages/ui/src/components/base/inputs/InputFrame.vue:4` transitions background-color, border-color, box-shadow and color | 100ms box-shadow ramp on focus and blur | none in `crates/sift-ui/src` (`crates/sift-ui/src/widgets/search.rs` draws focus with no ramp) | absent | OPEN | P1 |
| 451 | controls — copy chip | `packages/ui/src/components/base/CopyCode.vue:3` | `packages/ui/src/components/base/CopyCode.vue:3` `transition-[opacity,filter,transform,outline] duration-200 ease-in-out hover:brightness-[1.25] active:scale-95` | 200ms ease-in-out with a 0.95 press | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 452 | controls — dropdown chevron | `packages/assets/styles/classes.scss:448-469` | `packages/assets/styles/classes.scss:448-469` `transition: transform 0.125s ease-in-out` to `rotate(180deg)` when `aria-expanded` or popout-open | 125ms rotation on expand and collapse | none in `crates/sift-ui/src` (the kebab has no chevron) | absent | OPEN | P2 |
| 453 | overlays — popover panel (v-popper theme) | `packages/assets/styles/classes.scss:700-721` | `packages/assets/styles/classes.scss:700-721` wrapper at scale 0.85 and opacity 0, show on `transform 0.125s ease-in-out` and `opacity 0.125s ease-in-out`, hide `transform 0.0625s`; `:684-698` transform-origin per placement | 125ms scale plus fade in, 62.5ms out, origin at the anchor corner | `crates/sift-ui/src/pages/home.rs:791-793` (panel drawn when open) | panel appears and disappears in one frame; no scale, no fade, no origin | OPEN | P1 |
| 454 | overlays — button menu panel | `packages/ui/src/components/base/buttons/button-menu/ButtonMenuPanel.vue:43`<br>`packages/ui/src/styles/tailwind-utilities.css:56-74` | `packages/ui/src/components/base/buttons/button-menu/ButtonMenuPanel.vue:43` `<Transition name="floating-expand">`; `packages/ui/src/styles/tailwind-utilities.css:56-74` `opacity 0.125s var(--ease-out-expo), scale 0.125s var(--ease-out-expo)` from opacity 0 and scale 0.9 by 0.3, with `--floating-expand-origin` | 125ms ease-out-expo vertical unfold from the anchor edge | `crates/sift-ui/src/pages/home.rs:900-941` (menu drawn fully formed) | no unfold, no fade | OPEN | P1 |
| 455 | overlays — DropdownSelect list | `packages/ui/src/components/base/DropdownSelect.vue:319-341` | `packages/ui/src/components/base/DropdownSelect.vue:319-341` options enter and leave on `transform 0.2s ease` from `translateY(99.999%)`; `:240-244` trigger `transition: 0.05s` with `transition-delay: 0.2s` while open; `:267` arrow `transform 0.2s ease` | 200ms slide of the list, 200ms arrow rotation, trigger waits 200ms before its 50ms change | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 456 | overlays — FloatingPanel | `packages/ui/src/components/base/FloatingPanel.vue:269-292` | `packages/ui/src/components/base/FloatingPanel.vue:269-292` wraps the panel in a Transition, but `:298-315` every `floating-panel-*` rule is commented out | no animation: the class names resolve to nothing | `crates/sift-ui/src/shell.rs:438-460` and `crates/sift-ui/src/pages/home.rs:791-793` | none: both are untweened, so our behaviour matches the shipped reference | DONE | P2 |
| 457 | overlays — tooltip show delay | `packages/ui/src/providers/tooltip.ts:35` | `packages/ui/src/providers/tooltip.ts:35` SHOW_DELAY 300, applied at `:111-117`; `:112` skips the wait once a tooltip is already open | 300ms hover dwell, immediate for the next tooltip while one is showing | `crates/sift-ui/src/shell.rs:438-460` | pill mounts on the hovered frame; no 300ms dwell | OPEN | P1 |
| 458 | overlays — tooltip hide delay | `packages/ui/src/providers/tooltip.ts:36` | `packages/ui/src/providers/tooltip.ts:36` HIDE_DELAY 400, applied at `:140-149` | 400ms grace after the pointer leaves | `crates/sift-ui/src/shell.rs:438-460` | pill vanishes on the leave frame | OPEN | P1 |
| 459 | overlays — tooltip enter and leave | `packages/assets/styles/classes.scss:644-665` | `packages/assets/styles/classes.scss:644-665` enter `opacity 0.1s ease-out, scale 0.1s ease-out 50ms` from opacity 0 and scale 0.9; leave `opacity 0.08s ease-out, scale 0.08s ease-out` to opacity 0 and scale 1.1 | 100ms fade in with scale starting 50ms in; 80ms fade out while growing to 1.1 | none in `crates/sift-ui/src` | absent | OPEN | P1 |
| 460 | overlays — tooltip re-position | `packages/ui/src/components/floating/Tooltip.vue:271-276` | `packages/ui/src/components/floating/Tooltip.vue:271-276` `left 0.15s var(--ease-out-expo), top 0.15s var(--ease-out-expo)` while moving; `:104-107` a jump over 300px re-mounts via the `jumpKey` at `:253` instead of sliding | 150ms slide when the source moves, hard cut when it jumps | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 461 | toast — toast progress | `packages/ui/src/components/notifications/NotificationToast.vue:183` | `packages/ui/src/components/notifications/NotificationToast.vue:183` `transition-[left,width] duration-200 ease-in-out`; `:410-424` `notification-bottom-progress-waiting 1s linear infinite` from left -20% to 100% | 200ms width and left easing per update; 1s linear sweep while waiting | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 462 | toast — popup notifications | `packages/ui/src/components/nav/PopupNotificationPanel.vue:394-408` | `packages/ui/src/components/nav/PopupNotificationPanel.vue:394-408` enter, leave and move all 0.3s ease-in-out from `translateX(100%) scale(0.8)`, leave to `translateX(100%)`; `:410-418` downloads move and leave on `transform 0.3s ease-in-out` | 300ms slide in and out, 300ms stack re-flow | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 463 | loading — loading bar | `packages/ui/src/components/base/LoadingBar.vue:125`<br>`apps/app-frontend/src/App.vue:2160-2169` | `packages/ui/src/components/base/LoadingBar.vue:125` width 0.1s ease-in-out, height 0.1s ease-out, opacity 0.4s; `:61-63` progress advances every 100ms; `:28` duration 1000ms; `:88-93` hide 500ms after finish, reset 400ms later; `apps/app-frontend/src/App.vue:2160-2169` mounts it under the title bar | 1000ms fill in 100ms steps, 500ms dwell at 100%, 400ms fade | none in `crates/sift-ui/src` (no loading bar anywhere in `crates/sift-ui/src`) | absent | OPEN | P2 |
| 464 | loading — page loading bar | `packages/ui/src/components/base/PageLoadingBar.vue:7-14` | `packages/ui/src/components/base/PageLoadingBar.vue:7-14` width transition per phase, finish `0.1s ease-in-out`, creep `${creepDuration}ms linear`, fill `${duration}ms ease-in-out`; `:31-33` duration 900 and creepDuration 2000; `:154` opacity 0.4s | 900ms to 85%, 2000ms creep to 98%, 100ms finish, 400ms fade | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 465 | loading — skeleton shimmer | `packages/ui/src/components/base/LoadingIndicator.vue:48` | `packages/ui/src/components/base/LoadingIndicator.vue:48` placeholder `pop 4s ease-in-out infinite`; `:61` `shimmer 4s ease-in-out infinite`; `:64-86` delays 0s, 0.3s and 0.6s per row; `:89-112` pop fades opacity 0.25 to 0.5, shimmer travels translateX -80% to 80% | 4s loops with 300ms row stagger | none in `crates/sift-ui/src` | absent ⇒ renderer risk: animated gradient — 4s shimmer sweeping a linear-gradient with a 300ms row stagger | RENDERER-RISK | P2 |
| 466 | loading — skeleton label dots | `packages/ui/src/components/base/LoadingIndicator.vue:20-38` | `packages/ui/src/components/base/LoadingIndicator.vue:20-38` `dots 2s infinite` cycling empty, one, two, three dots | 2s text loop | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 467 | loading — progress bar | `packages/ui/src/components/base/ProgressBar.vue:100` | `packages/ui/src/components/base/ProgressBar.vue:100` width 0.2s ease-in-out; `:103-119` `progress-bar-waiting 1s linear infinite` from left -50% and width 20% to left 100% and width 20% | 200ms width easing; 1s linear indeterminate sweep | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 468 | loading — appearing progress bar | `packages/ui/src/components/base/AppearingProgressBar.vue:3-6` | `packages/ui/src/components/base/AppearingProgressBar.vue:3-6` enter-all 300ms ease-out and leave-all 200ms ease-in; `:24` `animate-pulse` fill with `transition-all duration-300 ease-out`; `:136` container opacity 0.3s ease | 300ms in, 200ms out, pulsing fill | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 469 | page — content reveal | `packages/ui/src/components/base/ReadyTransition.vue:22` | `packages/ui/src/components/base/ReadyTransition.vue:22` duration default 200, `:69` `Transition name="ready-fade" mode="out-in"`, `:80-88` opacity at the prop duration with ease-in-out; `:2-6` no enter transition on a cache hit | 200ms out-in cross-fade when a page's data resolves, none when content was cached | none in `crates/sift-ui/src` (pages in `crates/sift-ui/src/pages` return synchronously) | absent | OPEN | P2 |
| 470 | page — splash screen | `apps/app-frontend/src/components/ui/SplashScreen.vue:167-171` | `apps/app-frontend/src/components/ui/SplashScreen.vue:167-171` `transition: opacity 0.3s ease-in-out` from opacity 0 | 300ms fade out when the app becomes ready | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 471 | brand — wordmark rings | `packages/ui/src/components/brand/TextLogo.vue:43-60` | `packages/ui/src/components/brand/TextLogo.vue:43-60` `transition: transform 2s ease-in-out`, rings `spin 1s ease-in-out infinite forwards` and `spin 2s ease-in-out infinite reverse` | continuous counter-rotation, 1s and 2s, only when the `animate` prop is set | none in `crates/sift-ui/src` (`crates/sift-ui/src/brand.rs`) | no animated wordmark | OPEN | P2 |
| 472 | promo — ad slot and upgrade link | `apps/app-frontend/src/app/shell/sidebar/promotion.vue:61`<br>`apps/app-frontend/src/helpers/ads.js:3-8` | `apps/app-frontend/src/app/shell/sidebar/promotion.vue:61` `hover:underline` only; `:67-87` the ad body is a native window driven by `apps/app-frontend/src/helpers/ads.js:3-8` | no CSS transition on the promo; the only fade in that corner is the sidebar's 200ms bottom scroll fade (`sidebar/index.vue:145-148`, which shows when `showAd`); the ad window's own appearance is not stated in source | `crates/sift-ui/src/panel.rs:329-334` (banner painted statically) | no fade over the banner, no hover underline, no 200ms scroll fade | OPEN | P2 |
| 473 | disclosure — collapsible | `packages/ui/src/components/base/Collapsible.vue:98-107` | `packages/ui/src/components/base/Collapsible.vue:98-107` `grid-template-rows 0.3s ease-in-out`, `transition: none !important` on the first frame and under reduced motion; `:13` listens for `transitionend` | 300ms height animation, suppressed on mount so the first paint does not animate | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 474 | disclosure — collapsible region | `packages/ui/src/components/base/CollapsibleRegion.vue:63-72` | `packages/ui/src/components/base/CollapsibleRegion.vue:63-72` `grid-template-rows 500ms var(--ease-out-expo)`, disabled under reduced motion; `:15` scrim `transition-opacity duration-250`; `:27` chevron `transition-transform` with `rotate-180` | 500ms ease-out-expo open and close, 250ms scrim, chevron flip | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 475 | disclosure — accordion | `packages/ui/src/components/base/Accordion.vue:128-133`<br>`packages/ui/src/components/base/CollapsibleAdmonition.vue:44` | `packages/ui/src/components/base/Accordion.vue:128-133` `grid-template-rows 0.3s ease-in-out`; `:18` and `:36` chevron `transition-transform duration-300`; `packages/ui/src/components/base/CollapsibleAdmonition.vue:44` `transition-[grid-template-rows] duration-300 ease-in-out` and `:27` icon `duration-300` | 300ms height and chevron on the same curve | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 476 | disclosure — toggle card | `packages/ui/src/components/base/ToggleCard.vue:76` | `packages/ui/src/components/base/ToggleCard.vue:76` `grid-template-rows 0.25s var(--ease-out-expo)` | 250ms height reveal when the switch turns on | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 477 | misc — floating action bar | `packages/ui/src/components/base/FloatingActionBar.vue:259` | `packages/ui/src/components/base/FloatingActionBar.vue:259` `bottom 0.25s ease-in-out`, `:268-271` `floating-action-bar-attention 300ms ease-in-out`, `:288-294` transform and opacity 0.25s cubic-bezier(0.15, 1.4, 0.64, 0.96), `:321` animation none under reduced motion | 250ms slide with overshoot, 300ms attention nudge | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 478 | misc — terminal pulse | `packages/ui/src/components/base/BaseTerminal.vue:263-269` | `packages/ui/src/components/base/BaseTerminal.vue:263-269` `bpulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite`; `:314` opacity 250ms ease-in-out | 2s breathing overlay while the console streams | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 479 | misc — admonition progress | `packages/ui/src/components/base/Admonition.vue:89` | `packages/ui/src/components/base/Admonition.vue:89` `transition-[width] duration-200 ease-in-out`; `:231-236` `admonition-progress-waiting 1s linear infinite` | 200ms width easing, 1s linear sweep while waiting | none in `crates/sift-ui/src` | absent | OPEN | P2 |
| 480 | misc — scrollbar fade | `apps/app-frontend/src/assets/stylesheets/global.scss:126-135` | `apps/app-frontend/src/assets/stylesheets/global.scss:126-135` thumb `opacity 0.2s ease-in-out`, hover to opacity 1 | 200ms fade under the pointer | none in `crates/sift-ui/src` (the shell paints no scrollbar) | absent | OPEN | P2 |
| 481 | misc — scroll mask | `packages/ui/src/components/base/ScrollablePanel.vue:90-92` | `packages/ui/src/components/base/ScrollablePanel.vue:90-92` `--_top-fade-height 0.05s linear` and `--_bottom-fade-height 0.05s linear` | 50ms linear ramp of the mask gradient as content scrolls | none in `crates/sift-ui/src` | absent | OPEN | P2 |

## 14. Interaction / keyboard

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 482 | keyboard — `Escape` closes an open popover/menu | `UI/components/base/FloatingPanel.vue:224-229`<br>`UI/components/base/buttons/ContextMenu.vue:50`<br>`UI/components/base/buttons/TeleportPopoverMenu.vue:71` | `UI/components/base/FloatingPanel.vue:224-229` (panel) and `:215-219` (trigger); `UI/components/base/buttons/ContextMenu.vue:50`; `UI/components/base/buttons/TeleportPopoverMenu.vue:71` | — (this report has no states column) | no key is read for the row menu: `SRC/pages/home.rs:865-872` reacts only to a row *press*, and no `Escape` reaches it | an open kebab menu cannot be dismissed from the keyboard at all | OPEN | P1 |
| 483 | keyboard — `Escape` clears the library multi-selection | `APP/components/ui/library/use-library.ts:981-989` | `APP/components/ui/library/use-library.ts:981-989` (window-level `keydown`, honouring `defaultPrevented`) | — (this report has no states column) | no selection model exists; `SRC/pages/library.rs:296-320` paints cards with no `Sense` and no selection state | feature absent | OPEN | P2 |
| 484 | keyboard — `Escape` in a search field | `APP/features/friends/friends-list/index.vue:307`<br>`UI/components/base/Combobox.vue:940-942` | `APP/features/friends/friends-list/index.vue:307`: `@keyup.esc="search = ''"`; `UI/components/base/Combobox.vue:940-942` closes its dropdown | — (this report has no states column) | `SRC/widgets/search.rs:138-144` releases focus and keeps the text | reference *clears* the query on one surface; ours never clears on `Escape` | OPEN | P2 |
| 485 | keyboard — `Enter` in a search field | `UI/components/base/BaseTerminal.vue:33`<br>`APP/features/friends/friends-list/index.vue:274` | submit/confirm: `UI/components/base/BaseTerminal.vue:33`, `APP/features/friends/friends-list/index.vue:274` | — (this report has no states column) | `SRC/widgets/search.rs:123-133` releases focus, keeps text, commits nothing | no confirm action exists to fire; aligned by having nothing to submit, but the key does nothing visible | DONE | P3 |
| 486 | keyboard — keyboard control of a select/combobox | `UI/components/base/Combobox.vue:888-937`<br>`UI/components/base/DropdownSelect.vue:12-14` | `UI/components/base/Combobox.vue:888-937`; `UI/components/base/DropdownSelect.vue:12-14` | — (this report has no states column) | sort/kind selects: `SRC/shell.rs:1442-1457` click-to-toggle, options at `:1679-1694` click-to-choose; settings selects: `SRC/pages/settings.rs:2148-2156` click only | a select cannot be opened, walked or chosen from the keyboard | OPEN | P1 |
| 487 | keyboard — tab order between cards and their controls | `APP/components/ui/library/instance-group/index.vue:175-191` | `APP/components/ui/library/instance-group/index.vue:175-191` moves between cards; card controls are real `<button>`s so they are in the tab order | — (this report has no states column) | `SRC/pages/library.rs:307-319` paints cards; `SRC/pages/home.rs:696-700` paints cards; no focusable target exists | no tab order at all | OPEN | P2 |
| 488 | keyboard — arrow keys move between tabs | `UI/components/base/Tabs.vue:103-123` | `UI/components/base/Tabs.vue:103-123` (ArrowRight/Left wrap, Home/End, focus follows at `:98-100`) | — (this report has no states column) | project tabs `SRC/pages/project.rs:232-235`, instance tabs `SRC/pages/instance.rs:294-301`, skins segments `SRC/pages/skins.rs:996-1003` — click only | absent | OPEN | P2 |
| 489 | keyboard — Ctrl/Cmd+Z, Ctrl/Cmd+Shift+Z | `UI/layouts/shared/files-tab/composables/file-undo-redo.ts:84-88`<br>`use-image-editor.ts:1472-1479` | `UI/layouts/shared/files-tab/composables/file-undo-redo.ts:84-88`; image editor `use-image-editor.ts:1472-1479` | — (this report has no states column) | no editable document exists in `SRC` | not applicable today; will be needed for pack/instance editing | OPEN | P3 |
| 490 | keyboard — Ctrl/Cmd+Shift+I toggles the i18n debug panel | `UI/composables/i18n-debug.ts:241-246` | `UI/composables/i18n-debug.ts:241-246` | — (this report has no states column) | none | absent (dev-only surface) | OPEN | P3 |
| 491 | keyboard — Ctrl/Cmd+A in the terminal | `UI/components/base/BaseTerminal.vue:191` | `UI/components/base/BaseTerminal.vue:191` | — (this report has no states column) | no terminal in `SRC` | absent feature | OPEN | P3 |
| 492 | keyboard — hold `Alt` during a library drag to *add* to a group | `APP/components/ui/library/use-library.ts:785-797`<br>`APP/components/ui/library/instance-group/instance-group-dnd.vue:147-149` | `APP/components/ui/library/use-library.ts:785-797`; read per event at `APP/components/ui/library/instance-group/instance-group-dnd.vue:147-149`, `:151-167` | — (this report has no states column) | no drag model | absent | OPEN | P3 |
| 493 | keyboard — keybind capture swallows all keys, `Escape` clears | `APP/features/settings/syncing/game-settings-modal/keybind-input.vue:118-128, 165` | `APP/features/settings/syncing/game-settings-modal/keybind-input.vue:118-128, 165` | — (this report has no states column) | settings page is static by design: `SRC/pages/settings.rs:65` ("Controls are static") | absent | OPEN | P3 |
| 494 | context menu — library page background, right-click → `New instance` | `APP/pages/Index.vue:70-88` | `APP/pages/Index.vue:70-88` (only when `data-library-page-background` is the target), menu at `:104`, label `app.library.actions.label` = "Library actions" `:42-45` | — (this report has no states column) | none; `SRC/pages/library.rs:290-320` registers no secondary sense on the page | absent | OPEN | P2 |
| 495 | context menu — library group header → `New instance` / `Add to group` / `Edit group name` / `Delete group` | `APP/components/ui/library/instance-group/index.vue:326-360` | `APP/components/ui/library/instance-group/index.vue:326-360`, trigger `:498`, label "Group actions" `:283-286` | — (this report has no states column) | no groups exist; `SRC/pages/library.rs:290-320` draws one flat grid | absent | OPEN | P2 |
| 496 | context menu — instance page header → `Stop`/`Play`/`Add content`/`Edit`/`Open folder`/`Copy path` | `APP/pages/instance/layout.vue:5` | `APP/pages/instance/layout.vue:5` (`@contextmenu.prevent.stop`), `handleRightClick` `:816-864`, menu element `:85` | — (this report has no states column) | none; `SRC/pages/instance.rs:280-312` is tabs + tab body only | absent | OPEN | P2 |
| 497 | context menu — project page right-click | `APP/pages/project/Index.vue:68` | `APP/pages/project/Index.vue:68` (`@contextmenu.prevent.stop`), menu `:211-215`, label `messages.projectActionsLabel` | — (this report has no states column) | none; `SRC/pages/project.rs:232-249` | absent | OPEN | P2 |
| 498 | context menu — screenshot card → `Edit`/`Copy`/more; menu `Edit`/`Copy image`/`Show in folder`/`Go to instance`/`Delete` | `APP/components/ui/screenshots-page/card.vue:191-192`<br>`index.vue:1061`<br>`index.vue:1064-1092`<br>`index.vue:1432-1443` | `APP/components/ui/screenshots-page/card.vue:191-192`; event branch `index.vue:1061`; items `index.vue:1064-1092`; menu `index.vue:1432-1443`; strings `:273-278` | — (this report has no states column) | none; `SRC/pages/screenshots.rs:89` states "no dropdown, no hover and no press anywhere on this page" | absent | OPEN | P2 |
| 499 | context menu — world row → `Create shortcut` and friends | `APP/components/ui/world/WorldItem.vue:464-469` | `APP/components/ui/world/WorldItem.vue:464-469` (`@contextmenu.prevent.stop`), menu `:675`, item shown conditionally `:412-415`, string `instance.worlds.create_shortcut` `:255-256` | — (this report has no states column) | world rows only take a left press: `SRC/pages/home.rs:841-856` | absent | OPEN | P2 |
| 500 | context menu — quick-instance switcher row | `APP/components/ui/QuickInstanceSwitcher.vue:255, 310, 328` | `APP/components/ui/QuickInstanceSwitcher.vue:255, 310, 328` | — (this report has no states column) | no switcher in `SRC` | absent | OPEN | P3 |
| 501 | context menu — friends-list row | `APP/features/friends/friends-list/section.vue:40-101, 125-126` | `APP/features/friends/friends-list/section.vue:40-101, 125-126` | — (this report has no states column) | no friends UI | absent ⇒ blocked on the friends service — there is no friends list to hang a menu on | BLOCKED-SERVICE | P3 |
| 502 | context menu — browse result card | `UI/layouts/shared/browse-tab/layout.vue:300, 375`<br>`APP/pages/Browse.vue:1326, 1336-1340` | `UI/layouts/shared/browse-tab/layout.vue:300, 375`; `APP/pages/Browse.vue:1326, 1336-1340` | — (this report has no states column) | result cards are click-only: `SRC/shell.rs:1900-1912` | absent | OPEN | P2 |
| 503 | context menu — native context menu is suppressed except on images, textareas, contenteditable, text inputs, and text selections | `APP/App.vue:1931-1955` | `APP/App.vue:1931-1955`, listener `:493` | — (this report has no states column) | egui never shows an OS menu, so the visible outcome coincides; but none of the reference's exceptions (copy/paste on an image or a text field) exist, and there is no `Copy path` action | no native menu *and* no replacement menu on most targets | OPEN | P2 |
| 504 | context menu — menu accessibility scaffolding | `UI/components/base/buttons/ContextMenu.vue:34`<br>`button-menu.ts:72-81` | `UI/components/base/buttons/ContextMenu.vue:34` (id), `:68-69` (`panelElement.focus()`, selection cleared), `:97-107` (panel + role/label), `button-menu.ts:72-81` (`role=menuitem`, `tabindex=-1`) | — (this report has no states column) | none: rows are painted rects (`SRC/pages/home.rs:926-973`) | no roles, no focus move, no label, no announceable state | OPEN | P2 |
| 505 | tooltip — rail buttons show their destination on hover, placement right | `APP/App.vue:2014`<br>`UI/providers/tooltip.ts:348-359` | `APP/App.vue:2014` (Home), `:2025` (Discover), `:2036` (Skins), `:2043` (Screenshots), `:2049` (Modrinth hosting), `:2063` (create), `:2072` (Settings), `:2079` (profile); `.right` modifier → `right` placement at `UI/providers/tooltip.ts:348-359` | — (this report has no states column) | `SRC/shell.rs:764-814` (five destinations), `:842-858` (create), `:861-889` (settings/account): no hover text anywhere | every rail icon is unlabelled | OPEN | P1 |
| 506 | tooltip — show/hide timing | `UI/providers/tooltip.ts:35-36` | `SHOW_DELAY = 300` ms, `HIDE_DELAY = 400` ms (`UI/providers/tooltip.ts:35-36`), used at `:111` and `:140-149`; per-binding `delay` overrides at `:290-300` (object form `{ text, delay }` typed `:6`) | — (this report has no states column) | our only tooltip is `egui`'s `on_hover_text` at `SRC/pages/servers.rs:1832`; its delay is egui's own and is **not stated in our source** | delay/timing parity unverified | OPEN | P3 |
| 507 | tooltip — geometry: 8 px offset, flip, 8 px shift, arrow, default placement `top` | `UI/components/floating/Tooltip.vue:81`<br>`UI/providers/tooltip.ts:96` | `UI/components/floating/Tooltip.vue:81` (`offset(8)`, `flip()`, `shift({ padding: 8 })`, `arrow`), default `:54`, placement default `UI/providers/tooltip.ts:96` | — (this report has no states column) | `SRC/pages/servers.rs:1832` (egui default tooltip, no placement control) | no placement, offset, flip or arrow parity | OPEN | P3 |
| 508 | tooltip — a tooltipped target is made focusable and labelled | `UI/providers/tooltip.ts:273-274` | `UI/providers/tooltip.ts:273-274` calls `ensureFocusable` (+ `tabindex = 0` `:326-338`) and `ensureAriaLabel` (`:302-316`) | — (this report has no states column) | none | tooltips are the reference's *only* source of `aria-label` on icon-only buttons; we have neither | OPEN | P2 |
| 509 | tooltip — tooltip appears on keyboard focus only when focus is visible; hides on blur | `UI/providers/tooltip.ts:180-186` | `UI/providers/tooltip.ts:180-186` (`isFocusVisible`), `:206-219`, listeners `:240-252` | — (this report has no states column) | none | absent | OPEN | P3 |
| 510 | tooltip — instance card actions: `Loading`/`Installing`, `Stop`, `Repair`, `Play`, `Select`/`Deselect` | `APP/components/ui/library/instance-group/instance-card.vue:299, 314, 332, 349, 373` | `APP/components/ui/library/instance-group/instance-card.vue:299, 314, 332, 349, 373` | — (this report has no states column) | home card's pill and kebab: `SRC/pages/home.rs:726-781` paint only, no hover text | the two most-pressed controls in the library say nothing on hover | OPEN | P2 |
| 511 | tooltip — truncated name / loader+version show the full string | `APP/components/ui/library/instance-group/instance-card-view.vue:79, 86`<br>`APP/pages/instance/share/shared-instance-members-table.vue:79` | `APP/components/ui/library/instance-group/instance-card-view.vue:79, 86`; `APP/pages/instance/share/shared-instance-members-table.vue:79` | — (this report has no states column) | names are painted with no truncation and no tooltip: `SRC/pages/home.rs:705-723` | long names are either clipped silently or overflow — **not stated in our source** which | OPEN | P2 |
| 512 | tooltip — screenshot card: filename, `Edit`, `Copy`/`Copied`, `More actions` | `APP/components/ui/screenshots-page/card.vue:236, 249, 258, 268` | `APP/components/ui/screenshots-page/card.vue:236, 249, 258, 268` | — (this report has no states column) | `SRC/pages/screenshots.rs:800-820` (hover sense on the list only) | absent | OPEN | P3 |
| 513 | tooltip — group header actions: move up/down, rename, add, delete | `APP/components/ui/library/instance-group/group-action-buttons.vue:56, 66, 77, 86, 96` | `APP/components/ui/library/instance-group/group-action-buttons.vue:56, 66, 77, 86, 96` | — (this report has no states column) | no groups in `SRC` | absent | OPEN | P3 |
| 514 | tooltip — title-bar action bar (8 tooltips: downloads, view instance, stop, view logs …) | `APP/app/shell/title-bar/action-bar.vue:6, 27, 40, 64, 83, 90, 102, 109` | `APP/app/shell/title-bar/action-bar.vue:6, 27, 40, 64, 83, 90, 102, 109` | — (this report has no states column) | our title bar is the drag strip plus three window buttons: `SRC/shell.rs:714-735` | no action bar at all | OPEN | P3 |
| 515 | tooltip — world row: linked/synced server, incompatible version, ping, last played, play | `APP/components/ui/world/WorldItem.vue:502, 510, 539, 545, 598, 655` | `APP/components/ui/world/WorldItem.vue:502, 510, 539, 545, 598, 655` | — (this report has no states column) | `SRC/pages/home.rs:813-859` paints world rows; no hover text | absent | OPEN | P3 |
| 516 | tooltip — the one tooltip we do have | `ServersPromo.vue:57-64`<br>`SRC/pages/servers.rs:1809` | reference counterpart is the promo's `IconButton` label (our comment cites `ServersPromo.vue:57-64`, `SRC/pages/servers.rs:1809`); the precise reference line is **not stated in our source** | — (this report has no states column) | `SRC/pages/servers.rs:1826-1832` `.on_hover_text(PROMO_DISMISS_LABEL)` — the crate's only tooltip | none beyond timing/placement | DONE | P3 |
| 517 | drag & drop — drag an instance card between groups | `APP/components/ui/library/instance-group/instance-card.vue:112-121`<br>`APP/components/ui/library/instance-group/instance-group-dnd.vue:151-195`<br>`use-library.ts:643` | `APP/components/ui/library/instance-group/instance-card.vue:112-121` (`useDraggable`, disabled unless the Group view), handlers `APP/components/ui/library/instance-group/instance-group-dnd.vue:151-195`, drop-state rules `use-library.ts:643-…` | — (this report has no states column) | none | the whole Group view (and its only reorganisation gesture) is absent | OPEN | P2 |
| 518 | drag & drop — multi-selection rides along with the dragged card | `APP/components/ui/library/use-library.ts:603-631` | `APP/components/ui/library/use-library.ts:603-631` (all selected instances join the drag) | — (this report has no states column) | none | absent | OPEN | P3 |
| 519 | drag & drop — dragging over a *collapsed* group opens it after 750 ms | `APP/components/ui/library/instance-group/instance-group-dnd.vue:31` | `GROUP_HOVER_OPEN_DELAY = 750` `APP/components/ui/library/instance-group/instance-group-dnd.vue:31`, watcher `:128-138` | — (this report has no states column) | none | absent | OPEN | P3 |
| 520 | drag & drop — drag status popover follows the pointer, clamped to the window | `APP/components/ui/library/instance-group/instance-group-dnd.vue:75-78, 94-119` | `APP/components/ui/library/instance-group/instance-group-dnd.vue:75-78, 94-119` | — (this report has no states column) | none | absent | OPEN | P3 |
| 521 | drag & drop — reorder groups by dragging | `APP/components/ui/library/index.vue:5` | `APP/components/ui/library/index.vue:5` (`vuedraggable`), model `:98-111`, order write `:171-173`, list `:306` | — (this report has no states column) | none | absent | OPEN | P3 |
| 522 | drag & drop — drag screenshots into a group | `APP/components/ui/screenshots-page/index.vue:1281-1343` | `APP/components/ui/screenshots-page/index.vue:1281-1343` (dnd-kit) | — (this report has no states column) | none; cards are inert `SRC/pages/screenshots.rs:806-812` | absent | OPEN | P3 |
| 523 | drag & drop — drop a file on a file button / input / drop area | `UI/components/base/buttons/FileButton.vue:92-93`<br>`UI/components/base/FileInput.vue:2`<br>`UI/components/base/DropArea.vue:6-8`<br>`UI/components/base/DropzoneFileInput.vue:10-12` | `UI/components/base/buttons/FileButton.vue:92-93`; `UI/components/base/FileInput.vue:2`; `UI/components/base/DropArea.vue:6-8`; `UI/components/base/DropzoneFileInput.vue:10-12` | — (this report has no states column) | none; nothing in `SRC` reads dropped files | pack import and skin add both need this | OPEN | P2 |
| 524 | focus — focus ring | `STY/accessibility.scss:1-6`<br>`STY/variables.scss:222` | 0.25 rem (4 px) outline in `--color-focus-ring` (`STY/accessibility.scss:1-6`), value `#ea80ff` (`STY/variables.scss:222`) | — (this report has no states column) | no focus ring exists: no `request_focus`, no focus-dependent paint in `SRC` | keyboard users get no visible focus anywhere RE-VERIFIED 2026-11-01: `request_focus` is now called once, by Ctrl+F on the Content column's search field (widgets/search.rs:84-90), and its caret is the only focus signal in the app — no focus ring is drawn anywhere, which is the finding this row carries. | OPEN-WITH-CAUSE | P1 |
| 525 | focus — per-component focus rings (`focus-visible:ring-4` brand shadow) | `UI/components/base/Tabs.vue:17`<br>`STY/variables.scss:125` | `UI/components/base/Tabs.vue:17`; `--color-brand-shadow: rgba(0, 175, 92, 0.7)` `STY/variables.scss:125` | — (this report has no states column) | none | absent | OPEN | P2 |
| 526 | focus — body scroll is locked while a modal is open | `UI/components/modal/NewModal.vue:466` | `UI/components/modal/NewModal.vue:466`, released `:530` | — (this report has no states column) | egui has no document scroll; our pages keep their scroll state (`SRC/shell.rs:1569`) and are still live under the modal — whether a wheel behind the scrim scrolls is **not stated in our source** | UNKNOWN | OPEN | P3 |
| 527 | focus — `DropdownSelect` opens on focus and closes on blur | `UI/components/base/DropdownSelect.vue:8-10`<br>`UI/components/base/Combobox.vue:36-37` | `UI/components/base/DropdownSelect.vue:8-10`; combobox focus-in/out `UI/components/base/Combobox.vue:36-37` | — (this report has no states column) | none: our selects open only on a press (`SRC/shell.rs:1442-1452`) | absent | OPEN | P3 |
| 528 | focus — hover affordances are also reachable by focus | `UI/components/floating/FloatingMenu.vue:230-231, 246-247` | `UI/components/floating/FloatingMenu.vue:230-231, 246-247` (`@focusin`/`@focusout` show/hide) | — (this report has no states column) | hover-only: `SRC/shell.rs:791-795`, `SRC/pages/home.rs:769-781` | anything that only exists on hover is unreachable without a pointer | OPEN | P2 |
| 529 | pointer — click a card opens the instance; with a selection active, or with `Shift`, it toggles selection instead | `APP/components/ui/library/instance-group/instance-card.vue:148-154` | `APP/components/ui/library/instance-group/instance-card.vue:148-154`, card role/aria `:277-291` | — (this report has no states column) | the card body has no press at all — stated as a known hole at `SRC/pages/home.rs:737-739`, and `SRC/pages/library.rs:307-319` only paints cards | the reference's primary navigation gesture is missing; only the 5-row menu's `Edit` reaches the instance page RE-VERIFIED 2026-11-01: the card body's press opens the instance page (home.rs:776-780); the selection half of the row — Shift or an active selection toggling instead — is still absent. | OPEN-WITH-CAUSE | P0 |
| 530 | pointer — `Shift`/`Ctrl`/`Cmd`+click extends a selection | `UI/utils/modified-selection.ts:11-19`<br>`instance-card.vue:130-132` | `UI/utils/modified-selection.ts:11-19`; card passes `event.shiftKey` `instance-card.vue:130-132` | — (this report has no states column) | no selection model | absent | OPEN | P3 |
| 531 | pointer — play pill launches, and stops a running instance | `APP/components/ui/library/instance-group/instance-card.vue:178-205` | `APP/components/ui/library/instance-group/instance-card.vue:178-205` (play/stop), pill tooltip `:349` | — (this report has no states column) | `SRC/pages/home.rs:740-749` launches; there is no running-state pill (no stop) | half the affordance: a running instance cannot be stopped from the card | OPEN | P2 |
| 532 | pointer — kebab toggles its own popover; a second press closes it | `UI/components/base/FloatingPanel.vue:184-190` | `UI/components/base/FloatingPanel.vue:184-190` (`toggle`) | — (this report has no states column) | `SRC/pages/home.rs:782-790` toggles on the same kebab | matches | DONE | — |
| 533 | pointer — hover renders a quiet plate behind the kebab | `APP/components/ui/library/instance-group/instance-card.vue:309` | `APP/components/ui/library/instance-group/instance-card.vue:309` (`type="quiet"`) | — (this report has no states column) | `SRC/pages/home.rs:767-781` | matches | DONE | — |
| 534 | pointer — middle click never opens a tab; it is re-dispatched as a left click | `APP/App.vue:1917-1929` | `APP/App.vue:1917-1929`, listener `:492` | — (this report has no states column) | no middle-button sense is registered anywhere in `SRC` (`grep middle_clicked`: none) | same visible outcome by omission; no re-dispatch needed in a native window | DONE | P3 |
| 535 | pointer — double click | — (the report gives no file:line) | no `dblclick` handler exists in `APP` or `UI` (grep: no matches) | — (this report has no states column) | `SRC` has no double-click sense either | aligned by absence — nothing to port | DONE | P3 |
| 536 | pointer — pointing at a card probes whether the instance is running | `APP/components/ui/library/instance-group/instance-card.vue:289` | `APP/components/ui/library/instance-group/instance-card.vue:289` (`@mouseenter="checkProcess"`), handler `:172-176` | — (this report has no states column) | none: the running state is not consulted for a row (`SRC/pages/home.rs:726-736` always paints the play pill) | the card cannot show that an instance is already running | OPEN | P2 |
| 537 | pointer — window controls: hover re-paints the button, close turns red | `APP/app/shell/title-bar/window-controls.vue:7` | `APP/app/shell/title-bar/window-controls.vue:7` (drag-region exclude); hover colours resolve in the app stylesheet — exact rule **not stated in our source** | — (this report has no states column) | `SRC/shell.rs:714-735` (hover plate, red on close) | matches the measured capture; the reference's own hover selector is UNKNOWN | DONE | P3 |
| 538 | click-outside — a popover closes on a press outside it, ignoring its trigger and `#teleports` | `UI/components/base/FloatingPanel.vue:192-198` | `UI/components/base/FloatingPanel.vue:192-198` (`onClickOutside(panelRef, close, { ignore: [triggerRef, '#teleports'] })`) | — (this report has no states column) | `SRC/pages/home.rs:682-688`: a dismiss target over the *page rect*, registered before the rows so they win the press | a press on the rail, the title bar, the right panel or an open modal does not close the row menu | OPEN | P2 |
| 539 | click-outside — an open sort/kind select closes when you press elsewhere | `UI/components/base/Combobox.vue:1053`<br>`MultiSelect.vue:1423`<br>`DropdownFilterBar.vue:1911` | `UI/components/base/Combobox.vue:1053` (`onClickOutside`), `MultiSelect.vue:1423`, `DropdownFilterBar.vue:1911` | — (this report has no states column) | `SRC/shell.rs:1335-1342` is the placeholder for exactly this and does nothing (`let _ = &self.open_select;`); the only closers are another select's trigger (`:1447-1451`) or choosing an option (`:1690`) | an open select stays open over the results until it is used or re-pressed — a functional bug, not a polish gap | OPEN | P1 |
| 540 | scroll — the settings tab rail scrolls under the wheel | `TabbedModal.vue` (no line number in the report) | not stated in source (the reference rail is a plain scroll container; ``TabbedModal.vue`` scroll behaviour is UNKNOWN) | — (this report has no states column) | `SRC/pages/settings.rs:2397-2404` clamps a wheel-driven `rail_scroll` | our own addition; no reference value to match | OPEN | P3 |
| 541 | misc — a fullscreen console leaves fullscreen on `Escape` when no modal is open | `UI/layouts/shared/console/layout.vue:314` | `UI/layouts/shared/console/layout.vue:314` | — (this report has no states column) | no console in `SRC` | absent feature | OPEN | P3 |

## 15. File management

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 542 | Drag & drop — Listener for files dropped on the window | `af/providers/setup/file-drop.ts:23-62`<br>`ui/src/providers/file-drop.ts:3-19`<br>`ui/src/composables/file-drop.ts:136-156` | Tauri webview sends `enter`/`over`/`drop`/`leave` with paths + logical position through one provider (`af/providers/setup/file-drop.ts:23-62`, `ui/src/providers/file-drop.ts:3-19`); each drop target subscribes via `useFileDropTarget` (`ui/src/composables/file-drop.ts:136-156`) and only the target the pointer is inside accepts (`:47-62`). | — (this report has no states column) | No listener at all: the frame entry (`sift-ui/main.rs:79-91`) and the shell (`sift-ui/shell.rs:249`) never read `raw.dropped_files`, and the crate has no dialog or drop dependency (`crates/sift-ui/Cargo.toml:35-55`). | Whole ingest path missing: nothing can be dropped onto the window. RE-VERIFIED 2026-11-01: a drop is read where the Files panel draws and copied in (instance.rs:689-702 → drop.rs:29-53, three tests at drop.rs:71-88); there is still no window-level listener and no second surface that accepts a drop. | OPEN-WITH-CAUSE | P2 |
| 543 | Drag & drop — Where a dropped file lands — instance Content tab | `af/pages/instance/content/index.vue:1776-1784` | Each dropped path that is not `.mrpack` goes through `add_project_from_path(instance, path)` (`af/pages/instance/content/index.vue:1776-1784`); `.mrpack` is skipped explicitly (`:1780`); drops ignored while the instance is busy (`:1777`). | — (this report has no states column) | No drop handler; installs are started only from a project id on a card (`sift-ui/install.rs:156-187`, `:211-264`). | Dropping a jar into an instance imports nothing. | OPEN | P2 |
| 544 | Drag & drop — Where a dropped file lands — instance Files tab | `af/providers/setup/file-drop.ts:56-60`<br>`af/pages/instance/files/index.vue:238-270`<br>`layout.vue:616-619` | Native paths are turned into `File` objects (`af/providers/setup/file-drop.ts:56-60`) and written into the folder being browsed, by name (`af/pages/instance/files/index.vue:238-270`), via `layout.vue:616-619`. | — (this report has no states column) | The Files tab is a painted list of the instance root (`sift-ui/pages/instance.rs:470-510`) with no input of any kind. | Nothing lands anywhere in the Files tab. RE-VERIFIED 2026-11-01: a file dropped over the Files panel lands in the folder the panel is showing, through `drop::copy_into` (instance.rs:689-702; drop.rs:29-53), held by three unit tests (drop.rs:71-88); the drag itself is still undriven in this lab. | VERIFIED | P2 |
| 545 | Drag & drop — Drop highlight on a folder row | `ui/src/layouts/shared/files-tab/components/FileTableRow.vue:161-163`<br>`ui/src/layouts/shared/files-tab/composables/file-drag-state.ts:22-30` | The row under the pointer gets `!bg-brand-highlight` (`ui/src/layouts/shared/files-tab/components/FileTableRow.vue:161-163`, `:175-190`); the drag source dims to `opacity-50` (`:4`, `:164`); a floating name ghost follows the pointer (`ui/src/layouts/shared/files-tab/composables/file-drag-state.ts:22-30`, `:50-53`); 5-point threshold before a drag counts (`:16`). | — (this report has no states column) | No hover/drag state exists on any list row (`sift-ui/pages/instance.rs:479-509` paints text and icons only). | No highlight, ghost or drag affordance. | OPEN | P3 |
| 546 | Drag & drop — Overlay while dragging over the file table | `ui/src/layouts/shared/files-tab/components/upload/FileUploadDragAndDrop.vue:10-27`<br>`layout.vue:57-63`<br>`ui/src/utils/common-messages.ts:1096-1105` | `absolute inset-0 rounded-2xl bg-black/60` panel with a 64-point `UploadIcon` and `Drop {type} here to upload` (`ui/src/layouts/shared/files-tab/components/upload/FileUploadDragAndDrop.vue:10-27`, message id `files.upload.drag-and-drop.drop-to-upload` at `:52-57`); `type` is never passed by the layout (`layout.vue:57-63`), so `formatFileItemType` yields the plural — the overlay reads “Drop files here to upload” (`ui/src/utils/common-messages.ts:1096-1105`). | — (this report has no states column) | Nothing is drawn over any drop area. | No overlay or admonition for a drag in progress. | OPEN | P3 |
| 547 | Drag & drop — `.mrpack` opened from the OS (double-click / “open with”) | `af/App.vue:1507-1526`<br>`af/helpers/state.ts:42-48` | Arrives as the `RunMRPack` command, not as a window drop: `install_get_modpack_preview` then `install_create_modpack_instance`, with the unknown-pack warning modal (`af/App.vue:1507-1526`); the command is the shell's opening command (`af/helpers/state.ts:42-48`). | — (this report has no states column) | No command line, no single-instance open handler (`sift-ui/main.rs:1-91`); installs take a project id only (`sift-ui/install.rs:211-227`). | A local `.mrpack` cannot be installed at all. | OPEN | P2 |
| 548 | Drag & drop — No drop target outside those three places | `af/pages/instance/content/index.vue:1774`<br>`af/pages/Skins.vue:977`<br>`ui/src/layouts/shared/files-tab/components/upload/FileUploadDragAndDrop.vue:60-65` | Only three listeners exist: Content tab (`af/pages/instance/content/index.vue:1774`), Skins (`af/pages/Skins.vue:977`), Files tab (`ui/src/layouts/shared/files-tab/components/upload/FileUploadDragAndDrop.vue:60-65`). Worlds, Screenshots and the instance overview have none. | — (this report has no states column) | None anywhere. | n/a — matches the reference's set, all three missing. | OPEN | P3 |
| 549 | File pickers — Upload picker in the Files tab | `af/providers/setup/file-picker.ts:29-46`<br>`layout.vue:647-655` | `open({ multiple: true })` — no filters, no title, no starting directory stated in source (`af/providers/setup/file-picker.ts:29-46`); cancel returns `[]` (`:33`); falls back to a hidden `<input type=file multiple>` when no provider (`layout.vue:647-655`). | — (this report has no states column) | No picker: no `rfd`/`plugin-dialog` dependency exists (`crates/sift-ui/Cargo.toml:35-55`). | No way to add a file to an instance folder. | OPEN | P1 |
| 550 | File pickers — Save-as / download a single file | `af/pages/instance/files/index.vue:221-226`<br>`FileTableRow.vue:270-276` | `plugin:files\|file_save_as` for the browsed file (`af/pages/instance/files/index.vue:221-226`), menu label overridden to `Save as...` (`:36-39`, `:330`), shown only for files, not folders (`FileTableRow.vue:270-276`). | — (this report has no states column) | No save path; the Files tab has no menu (`sift-ui/pages/instance.rs:470-510`). | A file cannot be written out of an instance. | OPEN | P1 |
| 551 | File pickers — Import a local `.mrpack` | `af/providers/setup/file-picker.ts:58-79`<br>`af/.../ModpackStage.vue:214-221`<br>`file-picker.ts:66-69` | `open({ multiple:false, filters:[{ name:'Modpack', extensions:['mrpack'] }] })` (single file, filter `mrpack`) (`af/providers/setup/file-picker.ts:58-79`); cancel → `null` → no-op (`af/.../ModpackStage.vue:214-221`); `readFile:false` streams from the native path for instance imports (`file-picker.ts:66-69`). | — (this report has no states column) | Install resolves a *project* to its newest version over HTTP and downloads it (`sift-ui/install.rs:211-240`); `Job` carries only `project` + `title` (`:63-73`). | No local pack import; only Discover cards install. | OPEN | P1 |
| 552 | File pickers — Instance icon picker | `af/components/ui/library/use-library.ts:1188-1197`<br>`af/providers/setup/file-picker.ts:47-57` | Native `openDialog({ multiple:false, filters:[{ name:'Image', extensions:['png','jpeg','svg','webp','gif','jpg'] }] })` (`af/components/ui/library/use-library.ts:1188-1197`); cancel → `return` (`:1199`); then `edit_icon(id, path)` (`:1202`). A second provider offers the same extension set as `pickImage` (`af/providers/setup/file-picker.ts:47-57`). | — (this report has no states column) | The instance header omits the icon tile outright — `Instance` has no icon field (`sift-ui/pages/instance.rs:133-144`, shortcut comment at `:211-213`). | No icon, so nothing to pick. | OPEN | P2 |
| 553 | File pickers — Screenshots: export selection as ZIP | `af/components/ui/screenshots-page/index.vue:1226-1237` | `save({ defaultPath, filters:[{ name:'ZIP archive', extensions:['zip'] }] })` (`af/components/ui/screenshots-page/index.vue:1226-1237`); `defaultPath` is `Modrinth screenshots.zip` for the global page and `{instance} screenshots.zip` per instance, with `[\\/:*?"<>\|]` replaced by `-` (`:1229-1235`, copy at `:298-306`); cancel → `return` before any work (`:1237`). | — (this report has no states column) | No save dialog, no zip writer (`sift-ui/pages/screenshots.rs:77-93` lists the ZIP export as not done). | Screenshots cannot leave the app. | OPEN | P2 |
| 554 | File pickers — Instance → `.mrpack` export dialog | `af/components/ui/ExportModal.vue:96-105` | `save({ defaultPath: '<name> <version>.mrpack', filters:[{ name:'Modrinth Modpack', extensions:['mrpack'] }] })` (`af/components/ui/ExportModal.vue:96-105`); cancel → nothing written (`:107`); completion popup offers “Open in folder” on the written path (`:121-133`). | — (this report has no states column) | No export anywhere in the crate (no `.mrpack` writer; `sift-ui/install.rs` only reads packs). | Instance export is absent. | OPEN | P2 |
| 555 | File pickers — Starting directory / dialog title | `file-picker.ts:29-79`<br>`use-library.ts:1188-1197`<br>`index.vue:1231-1236`<br>`ExportModal.vue:97-105` | Not stated in source: every `open`/`save` call passes only `multiple`/`filters`/`defaultPath` (`file-picker.ts:29-79`, `use-library.ts:1188-1197`, `index.vue:1231-1236`, `ExportModal.vue:97-105`) — no `defaultPath` for opens, no `title` for any. | — (this report has no states column) | UNKNOWN (no dialogs exist). | n/a — nothing to match; a Windows `rfd` open would need a decision on the initial folder. | NOT-IN-SOURCE | P3 |
| 556 | Files tab — Per-instance root and refresh | `af/pages/instance/files/index.vue:65-67`<br>`ui/src/layouts/shared/files-tab/components/FileNavbar.vue:111-125` | The tab browses one instance through its own backend: `plugin:files\|file_list` (`af/pages/instance/files/index.vue:65-67`), root from `get_full_path` (`:46-54`), `show-refresh-button` on the layout (`:339`); refresh button in the navbar (`ui/src/layouts/shared/files-tab/components/FileNavbar.vue:111-125`, spinner for 1 s at `:333-341`). | — (this report has no states column) | Draws the instance folder read-only, top level only, capped at 24 rows (`sift-ui/pages/instance.rs:470-510`); no refresh, no path state (`State` holds just the tab, `:195-200`). | No refresh, no root/path state. | OPEN | P3 |
| 557 | Files tab — Browsing into folders | `ui/src/.../FileTableRow.vue:340-354`<br>`layout.vue:418-424`<br>`layout.vue:662-681` | Click (or Enter) on a directory navigates into it (`ui/src/.../FileTableRow.vue:340-354`), path joined in the layout (`layout.vue:418-424`); hover prefetches the folder after 150 ms (`layout.vue:662-681`). | — (this report has no states column) | Folders are rows of text with no hit target (`sift-ui/pages/instance.rs:470-510`); only the tab pills respond (`:294-301`). | There is no second level: the browser is one flat listing. RE-VERIFIED 2026-11-01: a folder row's press descends and the listing follows it (instance.rs:814-824, returned at :836), with the path bar to come back by (:715-763). | VERIFIED | P1 |
| 558 | Files tab — Breadcrumbs | `FileNavbar.vue:20-94` | Home icon button plus one segment per path part, `aria-current="location"` on the last, breadcrumb cursor `ChevronRightIcon` (`FileNavbar.vue:20-94`); overflow is measured with a `ResizeObserver` (`:352-358`) and masked/animated when too long (`:360-380`, styles `:439-464`). | — (this report has no states column) | No breadcrumb, no current-path state (`sift-ui/pages/instance.rs:195-200`). | No way to see or change location. RE-VERIFIED 2026-11-01: every ancestor is a target in a path bar over the listing, the root named for the directory the listing starts at (instance.rs:715-763). | VERIFIED | P2 |
| 559 | Files tab — Search inside the folder | `FileNavbar.vue:6-18`<br>`layout.vue:694-701` | A search `Input` (inline ≥800 px, its own row below) feeding `useFileSearch` (`FileNavbar.vue:6-18`, `:96-109`; `composables/file-search.ts`); cleared on navigation (`layout.vue:694-701`). | — (this report has no states column) | None (`sift-ui/pages/instance.rs:470-510`). | No filter over the listing. | OPEN | P2 |
| 560 | Files tab — Sorting | `FileTableHeader.vue:17-86`<br>`composables/file-sorting.ts:11-18` | Four sortable columns — name, size, created, modified — each toggling ascending/descending on re-click (`FileTableHeader.vue:17-86`, `composables/file-sorting.ts:11-18`); default `name` ascending (`:7-8`); directories always sort before files while the view is `all` (`:38-42`); size sorts a directory by its item count (`:49-53`). | — (this report has no states column) | Fixed order, folders first then name, no controls (`sift-ui/pages/instance.rs:632-633`). | No sort UI; the ordering rule is the only thing matched. | OPEN | P2 |
| 561 | Files tab — View filter (files only / folders only) | `composables/file-sorting.ts:9`<br>`layout.vue:693-701` | State exists (`composables/file-sorting.ts:9`, applied `:29-36`) but **no control anywhere calls it** — `resetSort` only (`layout.vue:693-701`); it is dead in 0.22.0. | — (this report has no states column) | None. | n/a — reference feature is unreachable; do not port. | NOT-IN-SOURCE | P3 |
| 562 | Files tab — Selection: per-row and select-all | `FileTableRow.vue:15-20`<br>`FileTableHeader.vue:12-16`<br>`layout.vue:694-701` | Checkbox per row (`FileTableRow.vue:15-20`); header checkbox with indeterminate state (`FileTableHeader.vue:12-16`); selection cleared on navigation (`layout.vue:694-701`); read-only rows make the bulk bar refuse (`:345-347`). | — (this report has no states column) | No selection state (`sift-ui/pages/instance.rs:195-200`). | Nothing can be selected. | OPEN | P2 |
| 563 | Files tab — Selection bar | `layout.vue:167-201` | `N selected`, Clear, Create ZIP (only when the host supplies `zipPaths`), Delete in red, disabled while busy (`layout.vue:167-201`). | — (this report has no states column) | None. | No bulk actions. | OPEN | P2 |
| 564 | Files tab — Range selection (Shift/Cmd click) | `composables/file-selection.ts:9-17`<br>`FileTableRow.vue:340-354` | Not stated in source: selection is a `Set` toggled per path only (`composables/file-selection.ts:9-17`); row click navigates/edits instead of selecting (`FileTableRow.vue:340-354`). | — (this report has no states column) | None. | n/a — reference has no range selection. | NOT-IN-SOURCE | P3 |
| 565 | Files tab — Rename | `ui/src/.../modals/FileRenameItemModal.vue:2-26`<br>`af/pages/instance/files/index.vue:156-173` | A modal, not in-place: header `Rename {name}`, prefilled input focused after 100 ms, submit disabled while invalid (`ui/src/.../modals/FileRenameItemModal.vue:2-26`, `:94-102`); validation — files `[a-zA-Z0-9-_.\s]+`, folders `[a-zA-Z0-9-_\s]+` (`:68-84`); rename maps to `file_rename` with the parent path preserved (`af/pages/instance/files/index.vue:156-173`). | — (this report has no states column) | No rename; the only writer in the crate is the install's settings stamp (`sift-ui/install.rs:285`). | Rename missing (modal semantics, not inline). RE-VERIFIED 2026-11-01: `rename_entry` exists and is tested (instance.rs:466-488, tests :1120-1151) but has no caller — `state.renaming` is passed into `files_tab` (:206, :346, :684) and never read. | OPEN-WITH-CAUSE | P1 |
| 566 | Files tab — Delete a single item — confirm | `ui/src/.../modals/FileDeleteItemModal.vue:2-22`<br>`layout.vue:478-485` | Critical `Admonition` modal, title `Delete file`, header `Deleting "{name}"`, separate file vs folder warning (“This folder and all its contents will be permanently deleted. This action cannot be undone.”) (`ui/src/.../modals/FileDeleteItemModal.vue:2-22`, `:40-57`); `recursive` is true only for directories (`layout.vue:478-485`). | — (this report has no states column) | No delete; `std::fs::remove*` appears only in settings/search/images/account code (`sift-ui/pages/settings.rs:337`, `sift-ui/search.rs:205`, `sift-ui/images.rs:220`, `sift-ui/account.rs:92`). | Delete and its confirmation are missing. RE-VERIFIED 2026-11-01: the row's `x` arms and the second press removes the file (instance.rs:782-813). | VERIFIED | P1 |
| 567 | Files tab — Delete a selection — confirm | `layout.vue:601-613` | None: `showBulkDeleteModal` deletes every selected path immediately, then deselects (`layout.vue:601-613`) — the name says modal but no modal is shown. | — (this report has no states column) | No delete. | Reference itself is unguarded; if we port the bar, add a confirm or record the shortcut. | OPEN | P2 |
| 568 | Files tab — Move to a typed destination | `ui/src/.../modals/FileMoveItemModal.vue:12-28`<br>`af/pages/instance/files/index.vue:175-186` | Modal with the current location shown and a free-text `Destination path` (`Example: /my-folder`), leading slash normalised on submit (`ui/src/.../modals/FileMoveItemModal.vue:12-28`, `:92-97`); the move is a `file_rename` (`af/pages/instance/files/index.vue:175-186`). | — (this report has no states column) | None. | No move-to-path. | OPEN | P2 |
| 569 | Files tab — Drag a row onto a folder | `composables/file-drag-state.ts:16`<br>`layout.vue:487-506` | Pointer-based: 5-point threshold, ghost label, `document.elementFromPoint` finds the nearest `[data-file-type="directory"]` (`composables/file-drag-state.ts:16`, `:32-37`); self-drop and drop-into-descendant are refused (`:93-104`); the move then goes through `handleDirectMove` (`layout.vue:487-506`). | — (this report has no states column) | None (`sift-ui/pages/instance.rs:479-509`). | No drag-to-folder. | OPEN | P2 |
| 570 | Files tab — Drag to reorder rows | `composables/file-sorting.ts:26-61`<br>`file-drag-state.ts:32-37` | Not stated in source — the list order is the sort order (`composables/file-sorting.ts:26-61`); the drag code has no insertion index, only a folder target (`file-drag-state.ts:32-37`). | — (this report has no states column) | None. | n/a — the reference does not reorder files. | NOT-IN-SOURCE | P3 |
| 571 | Files tab — Upload files into the current folder | `af/pages/instance/files/index.vue:228-270` | Sequential writes into `currentPath`, per-file progress (`currentFileName`, `uploadedBytes`, `completedFiles`), overwrite if present, refresh in `finally`, failure toast `uploadFailedLabel` (`af/pages/instance/files/index.vue:228-270`). | — (this report has no states column) | No upload (`sift-ui/pages/instance.rs:470-510`). | Cannot add a file. | OPEN | P1 |
| 572 | Files tab — New file / new folder | `ui/src/.../modals/FileCreateItemModal.vue:1-45`<br>`af/pages/instance/files/index.vue:135-154` | One modal parameterised by type (`Create a {type}`, placeholder `e.g. config.yml` / `e.g. my-folder`), same name validation as rename (`ui/src/.../modals/FileCreateItemModal.vue:1-45`, `:94-110`); a new file is written as zero bytes with `createOnly` (`af/pages/instance/files/index.vue:135-154`); folder create is its own backend call (`:139-142`). | — (this report has no states column) | None. | Neither create action exists. | OPEN | P1 |
| 573 | Files tab — “Create new…” menu | `FileNavbar.vue:127-192`<br>`af/pages/instance/files/index.vue:305-333` | `New file`, `New folder`, `Upload file`, divider, (`Upload from .zip` is `shown:false`), then `Upload from .zip URL` and `Install CurseForge pack` only when the host sets `showInstallFromUrl` (`FileNavbar.vue:127-192`); the instance page does not set it (`af/pages/instance/files/index.vue:305-333`). | — (this report has no states column) | None. | No create menu; the two hidden items are correctly out of scope for the instance. | OPEN | P2 |
| 574 | Files tab — Empty state | `layout.vue:114-127` | `FolderOpenIcon` 64-point, `This folder is empty`, `There are no files or folders.`, only when the folder is genuinely empty and not erroring (`layout.vue:114-127`, `:250-257`). | — (this report has no states column) | Falls back to the shared `stub` — tab name plus one sentence (`sift-ui/pages/instance.rs:472-475`, `stub` at `:647-668`). | Copy differs and there is no icon/centred layout. | OPEN | P2 |
| 575 | Files tab — Error state | `layout.vue:128-135`<br>`FileManagerError.vue:1-20` | `Unable to load files` / `The folder may not exist.` with `Try again` and `Go to home folder` (`layout.vue:128-135`, `:258-265`; `FileManagerError.vue:1-20`, `:31-40`). | — (this report has no states column) | None: an unreadable folder returns an empty list and reads as empty (`sift-ui/pages/instance.rs:619-622`). | A broken folder is indistinguishable from an empty one. | OPEN | P2 |
| 576 | Files tab — Row actions / context menu | `ui/src/.../FileTableRow.vue:200-287` | Right-click and the row's `…` open the same list: Copy filename, Copy full path, Open in folder (when the host supplies it), Extract (`.zip` only), Create ZIP (folder only), Rename, Move, Download (files only), Delete (red), each write action carrying a read-only tooltip (`ui/src/.../FileTableRow.vue:200-287`). | — (this report has no states column) | None. | No context menu. | OPEN | P1 |
| 577 | Files tab — Extract a `.zip` | `layout.vue:530-569`<br>`modals/FileUploadConflictModal.vue:88-114`<br>`layout.vue:278-281` | Dry run first; no conflicts → extract; conflicts → `Extract summary` modal listing `Will overwrite <path>` with an Overwrite confirm (`layout.vue:530-569`; `modals/FileUploadConflictModal.vue:88-114`); success toast `Extraction started` (`layout.vue:278-281`). | — (this report has no states column) | None. | No archive handling. | OPEN | P2 |
| 578 | Files tab — Create a ZIP | `FileTableRow.vue:244-252`<br>`layout.vue:178-187`<br>`modals/FileCreateZipModal.vue:41-56` | Folder’s `Create ZIP` (`FileTableRow.vue:244-252`) or the selection bar (`layout.vue:178-187`), then `Archive name` modal validating single name and existing file (`modals/FileCreateZipModal.vue:41-56`). | — (this report has no states column) | `flate2` is a dependency (`crates/sift-ui/Cargo.toml:54`) but only `images.rs`/`install` use it — no zip write. | No zip creation. | OPEN | P3 |
| 579 | Files tab — Read-only `mods/` | `af/pages/instance/files/index.vue:69-75`<br>`layout.vue:93-96` | `mods/` is read-only with the reason `Manage your installed content via the Content tab` (`af/pages/instance/files/index.vue:69-75`, `:32-35`), enforced on every write control (`layout.vue:93-96`, `:314-316`). | — (this report has no states column) | None. | Not modelled. | OPEN | P2 |
| 580 | Files tab — Text/image editor | `ui/src/utils/file-extensions.ts:95-98`<br>`components/editor/FileEditor.vue:201-205`<br>`layout.vue:148-166` | Code and text and image extensions open in the editor (`canOpenInFileEditor`, `ui/src/utils/file-extensions.ts:95-98`, lists at `:2-42`); save/revert, unsaved-changes floating bar, find & replace, share-to-mclogs for logs (`components/editor/FileEditor.vue:201-205`, `:243-280`, `:301-386`; bar at `layout.vue:148-166`). | — (this report has no states column) | None. | No editing. | OPEN | P2 |
| 581 | Files tab — Undo / redo rename and move | `composables/file-undo-redo.ts:19-92`<br>`layout.vue:704-710` | Ctrl/Cmd+Z and Ctrl/Cmd+Shift+Z over an operation history, with toasts (`composables/file-undo-redo.ts:19-92`), listener registered on the document in `layout.vue:704-710`. Unrelated to the editor's own revert. | — (this report has no states column) | None. | No undo. | OPEN | P3 |
| 582 | Files tab — Long-list handling | `layout.vue:357-366`<br>`FileTableHeader.vue:2-10` | Virtual scroll, 61-point rows, 5-row buffer, sticky header measured with an observer (`layout.vue:357-366`, `:369-371`; `FileTableHeader.vue:2-10`). | — (this report has no states column) | Hard cap of 24 rows with a break when the card fills (`sift-ui/pages/instance.rs:479-481`). | Lists truncate silently. | OPEN | P3 |
| 583 | Files tab — Per-folder row icons | `FileTableRow.vue:289-301` | `config`→FolderCog, `world`/`saves`→Globe, `mods`→Box, `resourcepacks`→Paintbrush, `shaderpacks`→Glasses, `datapacks`→Braces, else Folder; files by extension (`FileTableRow.vue:289-301`). | — (this report has no states column) | One `Folder`/`File` glyph (`sift-ui/pages/instance.rs:483-491`). | Icons are generic. | OPEN | P3 |
| 584 | Screenshots — Global page lists every instance | `af/pages/Screenshots.vue:27`<br>`af/components/ui/screenshots-page/index.vue:322-328`<br>`af/pages/instance/query-options.ts:56-62` | `/screenshots` renders the same page with no `instance-id` (`af/pages/Screenshots.vue:27`), so the list query is the all-instances one (`af/components/ui/screenshots-page/index.vue:322-328` → `list_synced_screenshots`, `af/pages/instance/query-options.ts:56-62`). | — (this report has no states column) | `read_shots` walks every instance's `screenshots/` for png/jpg (`sift-ui/pages/screenshots.rs:1549-1608`). | Listing matches; everything below does not. | DONE | P3 |
| 585 | Screenshots — Copy image to clipboard | `index.vue:1248-1271` | Reads the PNG from `screenshot.path` and writes a `ClipboardItem`, then marks the card “Copied” for 2 s (`index.vue:1248-1271`; labels `:273-274`; per-card button `:1454-1460`). | — (this report has no states column) | None; the crate never touches a clipboard (`grep clipboard` finds only doc comments, `sift-ui/widgets/search.rs:20`). | No copy to clipboard. RE-VERIFIED 2026-11-01: this build has no image clipboard — egui's carries text, which is why the card's menu carries only the rows the page can honour (screenshots.rs:79-81, :558-561). | OPEN-WITH-CAUSE | P2 |
| 586 | Screenshots — Show in folder | `index.vue:1273-1279`<br>`af/helpers/instance.ts:277-279` | `open_screenshot(key)` from the options menu (`index.vue:1273-1279`, `:1077-1082`; backend `af/helpers/instance.ts:277-279`). | — (this report has no states column) | None. `open_instance_folder` exists but opens the instance root, not a file (`sift-ui/shell.rs:1058-1074`). | A screenshot cannot be revealed. RE-VERIFIED 2026-11-01: the menu's Show in folder calls `reveal_in_folder` (`screenshots.rs`: Item at :437, function at :2004-2024), which selects the file in the platform's file manager. | VERIFIED | P2 |
| 587 | Screenshots — Edit (crop / annotate), save copy vs replace | `ui/src/components/image-viewer-editor/image-viewer-editor-messages.ts:8-32`<br>`ui/src/components/image-viewer-editor/types.ts:26-29`<br>`af/helpers/instance.ts:215-225`<br>`index.vue:707-720` | Viewer → editor with a `Crop` tool and save menu `Save copy` vs `Save image` (`ui/src/components/image-viewer-editor/image-viewer-editor-messages.ts:8-32`; payload mode `create_copy \| replace_edit` at `ui/src/components/image-viewer-editor/types.ts:26-29`), persisted by `save_edited_screenshot` (`af/helpers/instance.ts:215-225`; call site `index.vue:707-720`). | — (this report has no states column) | None. | No viewer, no editor, no write-back. | OPEN | P2 |
| 588 | Screenshots — Export selection as ZIP | `index.vue:1685-1693`<br>`af/helpers/instance.ts:266-268` | Bar button → save dialog → `export_screenshots(keys, path)` (`index.vue:1685-1693`, `:1226-1246`; backend `af/helpers/instance.ts:266-268`). | — (this report has no states column) | None (listed as missing at `sift-ui/pages/screenshots.rs:91`). | No export. | OPEN | P2 |
| 589 | Screenshots — Custom groups | `index.vue:874-980` | New group from the toolbar or selection bar, inline rename, delete with description, remove-from-group (`index.vue:874-980`, `:1583`, `:1665-1684`; copy `:250-317`). | — (this report has no states column) | Month buckets only (`sift-ui/pages/screenshots.rs:577-618`). | No user groups. | OPEN | P3 |
| 590 | Screenshots — Go to the owning instance | `index.vue:1100-1102` | Options menu `Go to instance` routes to `/instance/<id>` (`index.vue:1100-1102`, `:1083-1087`). | — (this report has no states column) | The instance name is painted on each card's caption (`sift-ui/pages/screenshots.rs:1265-1354`, caption at `:1322`, helper `instance_and_date` at `:652`); no press. | No navigation from a screenshot to its instance. | OPEN | P3 |
| 591 | Worlds — Per-world menu | `af/components/ui/world/WorldItem.vue:328-424`<br>`worlds/index.vue:105`<br>`af/helpers/worlds.ts:220-223` | Play/stop, refresh, copy address (servers), Edit, Open folder, Don’t show on home, Create shortcut, Desync, Delete (`af/components/ui/world/WorldItem.vue:328-424`); Open folder calls `showWorldInFolder` (`worlds/index.vue:105`; `af/helpers/worlds.ts:220-223`). | — (this report has no states column) | None — no hit targets in the tab (`sift-ui/pages/instance.rs:438-466`). | No world actions at all. RE-VERIFIED 2026-11-01: a world row is a hit target that launches the instance into that world (instance.rs:655-663, returned at :673) — the menu itself, Play/stop, Edit, Open folder, Create shortcut and Delete, is still absent. | OPEN-WITH-CAUSE | P1 |
| 592 | Worlds — Delete with confirmation | `af/components/ui/world/modal/ConfirmRemoveWorldModal.vue:25-75`<br>`worlds/index.vue:702-715`<br>`af/helpers/worlds.ts:151-153` | Modal with two copies: `Delete world` + “This world and all its contents will be permanently deleted…” for singleplayer, `Remove server` with `Remove here` / `Remove everywhere` for a synced server (`af/components/ui/world/modal/ConfirmRemoveWorldModal.vue:25-75`; dispatch `worlds/index.vue:702-715`; backend `af/helpers/worlds.ts:151-153`, `:189-194`). | — (this report has no states column) | None. | No delete, no confirm. | OPEN | P1 |
| 593 | Worlds — Edit a world | `af/components/ui/world/modal/EditSingleplayerWorldModal.vue:18`<br>`af/helpers/worlds.ts:121-145` | Rename (`rename_world`), reset icon (`reset_world_icon`), display status (`set_world_display_status`) from one modal (`af/components/ui/world/modal/EditSingleplayerWorldModal.vue:18`, `:43-44`; helpers `af/helpers/worlds.ts:121-145`). | — (this report has no states column) | None. | No rename/reset. | OPEN | P2 |
| 594 | Worlds — Import / export / duplicate a world | `af/helpers/worlds.ts:147-149` | **Not stated in source** — no import, export or duplicate control exists in the Worlds tab, and no helper names one. `backup_world` is defined (`af/helpers/worlds.ts:147-149`) but has **no call site** in `af/` or `ui/`. | — (this report has no states column) | None. | n/a for parity; a world import/export is a product decision, not a port. | NOT-IN-SOURCE | — |
| 595 | Instance icon — Pick / replace / remove | `af/components/ui/library/use-library.ts:1234-1262` | Menu offers `Select icon` (or `Replace icon`), `Create icon` (or `Edit created icon`), and `Remove icon` when one exists (`af/components/ui/library/use-library.ts:1234-1262`); pick = native image dialog then `edit_icon(id, path)` (`:1188-1207`); remove = `edit_icon(id, null)` (`:1209-1216`). | — (this report has no states column) | No icon field on `Instance` (`sift-ui/pages/instance.rs:133-144`), and the library card has no icon to show or set (`sift-ui/pages/library.rs:249-252`, `:571-572`). | Nothing to pick, replace or remove. | OPEN | P2 |
| 596 | Instance icon — Crop | `use-library.ts:1188-1203`<br>`ui/src/components/image-viewer-editor/image-viewer-editor-messages.ts:19-32` | **Not stated in source** for instance icons: the picked path is stored as-is (`use-library.ts:1188-1203`). Cropping exists only in the screenshot editor (`ui/src/components/image-viewer-editor/image-viewer-editor-messages.ts:19-32`). | — (this report has no states column) | None. | n/a — there is no icon crop in the reference to port. | NOT-IN-SOURCE | P3 |
| 597 | Instance icon — Generated icon editor | `af/components/ui/instance_settings/icon-editor-modal/index.vue:250-299` | Modal with `Background` swatches, `Symbol` grid (vanilla + modded), `Recents`, `Randomize`, `Save icon`; saved through `edit_generated_icon` / `cache_generated_icon` (`af/components/ui/instance_settings/icon-editor-modal/index.vue:250-299`; copy `:303-328`; `randomizeAndSave` `:271-284`). | — (this report has no states column) | None. | No generated icons. | OPEN | P3 |

## 16. Renderer

| # | element | reference (file:line) | reference value / behaviour | states & animation | ours (file:line) | gap | status | priority |
|---:|---|---|---|---|---|---|---|---|
| 598 | Renderer — Window creation, title, inner/min size | plumbing.md row 1 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Create a decorated, resizable winit window at 1440x900 logical, min 960x540 logical, title "Sift Launcher", default decoration, and drive one 1440x900 UI inside it | in use: yes | `src/main.rs:44-51`<br>`src/geometry.rs:35` | risk if missed: Window opens at the wrong size, cannot be titled, or is un-resizable when the app expects the OS frame. | OPEN | P0 |
| 599 | Renderer — Max size, resizability, decorations, window icon, app id | plumbing.md row 2 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Keep decorations ON so the app's painted title bar stays under the OS one; supply no icon; no app-id (Wayland falls back to the title) | in use: no (defaults only) | `src/main.rs:44`<br>`src/pages/settings.rs:1337-1342` | risk if missed: Turning decorations off makes the painted title bar the only one, and it cannot move the window (row 5): the app becomes unmovable. | OPEN | P1 |
| 600 | Renderer — Viewport-size awareness / responsive layout | plumbing.md row 3 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Render into a fixed 1440x900 point space anchored top-left and paint the remainder with the clear colour; do not scale-to-fit and do not reflow | in use: no | `src/geometry.rs:35`<br>`src/shell.rs:292`<br>`src/shell.rs:579`<br>`src/shell.rs:863-867`<br>`src/pages/mod.rs:70-84`<br>`src/pages/settings.rs:2502` | risk if missed: Any resize gives a cut-off window; a renderer that scales-to-fit breaks every measured edge and every +/-2 px parity claim. | OPEN | P0 |
| 601 | Renderer — Custom title bar contents (mark, wordmark, breadcrumb, status, controls) | plumbing.md row 4 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Text+galleys, one textured quad (logo), three icon meshes, hover plates, and three `interact` hit rects at 44x48 | in use: yes | `src/shell.rs:578-700` | risk if missed: No wordmark, route or status line; the three controls stop even hovering. | OPEN | P0 |
| 602 | Renderer — Title-bar drag region and double-click-to-maximise | plumbing.md row 5 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Nothing today -- the window is dragged by the OS frame that row 2 keeps on. If the frame ever goes away, this must be added first | in use: NO | No `Sense::drag` on the bar, no `ViewportCommand::StartDrag`, no `double_clicked` anywhere (grep: 0 hits) | risk if missed: With decorations off, the window cannot be moved at all; double-click does nothing. | DONE | P2 |
| 603 | Renderer — Minimise / maximise / close buttons | plumbing.md row 6 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Either wire the three presses to winit (`set_minimized`, `set_maximized`, close) or keep drawing three dead buttons | in use: drawn, NOT wired | `src/shell.rs:670-700` | risk if missed: Three buttons in the corner of every window that do nothing; this is what ships today, so a replacement must at least not regress the hover. | OPEN | P1 |
| 604 | Renderer — HiDPI: scale factor, pixels_per_point, zoom | plumbing.md row 7 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Multiply the whole 1440x900 point space by winit's `ScaleFactorChanged` factor, and rasterise glyphs at that factor (not upscale a 1x atlas); expose no zoom, because the app has none | in use: no | `src/fonts.rs:20-30`<br>`src/geometry.rs:35` | risk if missed: On a 2x display the UI is a quarter-size island, or blurry if glyphs are rasterised once at 1x. | OPEN | P1 |
| 605 | Renderer — Text input: the hand-rolled search field | plumbing.md row 8 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Deliver `Event::Text` for composed characters (IME and dead keys already arrive as text), key events with a press/release flag, and a monotonic frame clock | in use: yes | `src/widgets/search.rs:78` | risk if missed: No typing at all; or two characters deleted per press if press and release are both applied (the code comment calls this out). | OPEN | P0 |
| 606 | Renderer — Text input: `egui::TextEdit` (the fields the app does not own) | plumbing.md row 9 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | A full editor: selection, shift-motion, Home/End, Ctrl+A/C/V/X, IME pre-edit, caret positioned by the shaper, scroll-to-caret, and a `changed` signal | in use: yes | `src/pages/settings.rs:2896`<br>`src/pages/servers.rs:1354` | risk if missed: The biggest invisible loss: paste and selection die in every settings field and in the New server dialog. | OPEN | P0 |
| 607 | Renderer — Keyboard focus traversal (Tab) | plumbing.md row 10 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Deliver keys only, and keep the `TextEdit` ids stable between frames (they are keyed by label/placeholder) | in use: no | `src/widgets/search.rs:61` | risk if missed: A renderer that installs a focus graph steals keys from a field the app still thinks is focused; one that does not must not regenerate widget ids per frame or typing is lost. | OPEN | P1 |
| 608 | Renderer — Modifiers (shift/ctrl/alt) | plumbing.md row 11 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Pass modifier state through with every key event so the editor and shortcuts can use it | in use: indirect | `src/pages/home.rs:597`<br>`src/pages/about.rs:140`<br>`src/pages/settings.rs:2424` | risk if missed: Ctrl+C in a field becomes a literal "c"; Shift+click selection stops. | OPEN | P1 |
| 609 | Renderer — Clipboard | plumbing.md row 12 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Implement `set_text` and `get_text` against the platform clipboard (X11 + Wayland on Linux) | in use: write yes, read only via TextEdit | `src/pages/settings.rs:2478` | risk if missed: "Copy ad details" does nothing and paste into every field stops working. | OPEN | P1 |
| 610 | Renderer — File drag-and-drop onto the window | plumbing.md row 13 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Nothing to preserve. eframe feeds winit's drag events into `RawInput` and no call site reads them; a replacement may skip it, or convert them for a future import | in use: NO | `src/shell.rs:913` | risk if missed: Nothing today. The risk is only that a future Import is written against an input the replacement never delivers. | DONE | P3 |
| 611 | Renderer — File dialogs | plumbing.md row 14 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Clicks and keys only. Do not add a dialog crate: there is no call site | in use: no | `src/pages/settings.rs:2204-2365` | risk if missed: A new dependency with no user, and a modal that fights the app's own picker overlay. | DONE | P2 |
| 612 | Renderer — OS integration: shell-out, own settings store, spawns | plumbing.md row 15 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Nothing from the renderer, but do not change the working directory or the environment the spawn inherits, and never block the frame thread on it | in use: yes | `src/shell.rs:1022-1032`<br>`src/pages/settings.rs:203`<br>`src/account.rs:8` | risk if missed: "Open folder" opens the wrong place; a blocking spawn freezes the window while the file manager runs. | OPEN | P1 |
| 613 | Renderer — Texture upload: sizes, formats, filtering | plumbing.md row 16 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | RGBA8 upload, per-texture LINEAR vs NEAREST, a texture-id allocator, and epaint's premultiplied-alpha blend | in use: yes | `src/brand.rs:130`<br>`src/images.rs:126`<br>`src/pages/servers.rs:1081`<br>`src/pages/skins.rs:1368` | risk if missed: NEAREST ignored blurs pixel-art skins; a premultiply mismatch rims every icon with a dark halo. ⇒ renderer risk: premultiplied-alpha blending and per-texture LINEAR/NEAREST filtering | RENDERER-RISK | P0 |
| 614 | Renderer — Texture destroy (eviction) | plumbing.md row 17 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Free the GPU texture when the handle drops; never update in place (the app never does) | in use: yes, implicit | `src/images.rs:43` | risk if missed: One leaked texture per icon scrolled past, for the life of the process: the memory bound the module documents stops holding. | OPEN | P1 |
| 615 | Renderer — Texture draw (UV sub-rect, tint) | plumbing.md row 18 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | A textured quad with a UV sub-rectangle, a tint colour, and clip-rect intersection | in use: yes | `src/brand.rs:102`<br>`src/shell.rs:1697`<br>`src/pages/project.rs:118`<br>`src/pages/servers.rs:1676`<br>`src/pages/instance.rs:412`<br>`src/pages/skins.rs:838` | risk if missed: Skin faces draw the wrong crop of the atlas; tinted icons paint at full white. | OPEN | P0 |
| 616 | Renderer — Repaint on demand (`request_repaint`) | plumbing.md row 19 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | A wakable event loop (`ControlFlow::Wait`) plus a post-a-user-event path from `request_repaint`, callable off the frame thread | in use: yes, central | `src/main.rs:88`<br>`src/shell.rs:485-495`<br>`src/shell.rs:424`<br>`src/pages/settings.rs:2295`<br>`src/images.rs:166` | risk if missed: Either 100% idle CPU (poll every vsync) or a window that stops updating after the next press -- the LIGHTNESS budget the crate is built around. | OPEN | P0 |
| 617 | Renderer — Repaint after a delay (`request_repaint_after`) | plumbing.md row 20 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | A timer that wakes the loop at a deadline -- not "next vsync" | in use: yes | `src/pages/settings.rs:2482`<br>`vendor/fastframe-scroll/src/lib.rs:166` | risk if missed: The Copied label never returns; trackpad gestures never detect the lift, so glides never start or never stop. | OPEN | P1 |
| 618 | Renderer — Continuous vs on-demand policy | plumbing.md row 21 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Sleep when nothing asked for a frame, and paint one frame on any input event (no call site asks for a frame because a key or pointer arrived) | in use: on demand | `src/main.rs:82-90`<br>`src/widgets/search.rs:23-36`<br>`src/panel.rs:331-334` | risk if missed: Hovering and clicking paint nothing: the app never asks for a frame on input, so the renderer must. | OPEN | P0 |
| 619 | Renderer — Window-state persistence | plumbing.md row 22 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Nothing. Do not begin persisting window geometry the app never reads | in use: no | `Cargo.toml:36-41`<br>`src/pages/settings.rs:203` | risk if missed: No regression from the renderer; but dropping the app's own store path loses every setting silently. | DONE | P3 |
| 620 | Renderer — Cursor shape | plumbing.md row 23 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Nothing. An arrow everywhere matches today exactly | in use: NO | 0 hits for `CursorIcon`/`set_cursor_icon`: no I-beam, hand or resize cursor anywhere | risk if missed: None today (a missed nicety). A renderer that sets `None` and gets winit's default right is correct. | DONE | P3 |
| 621 | Renderer — Painting engine: tessellation, AA, meshes, clipping | plumbing.md row 24 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Tessellate rounded rects and stroked paths with antialiasing, vertex-coloured triangle meshes, gamma-correct blending, and a clip-rect stack | in use: yes, everywhere | `src/settings.rs:2680`<br>`src/pages/skins.rs:946`<br>`src/shell.rs:246-266`<br>`src/icons.rs:2600-2620`<br>`src/icons.rs:2632-2645` | risk if missed: The entire look: a rect-only softbuffer renderer loses all 441 icon glyphs, every rounded corner and every AA edge. ⇒ renderer risk: antialiased rounded-rect and path tessellation, vertex-coloured meshes, gamma-correct blending, clip stack | RENDERER-RISK | P0 |
| 622 | Renderer — Text shaping, glyph atlas, font fallback | plumbing.md row 25 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | A shaper (egui 0.36 uses `harfrust`), a glyph rasteriser, an atlas texture, the fallback chain, and both wrap and no-wrap layout | in use: yes | `src/fonts.rs:38-70` | risk if missed: No text; or text without the Inter weights the whole theme was measured against. | OPEN | P0 |
| 623 | Renderer — Input translation: pointer, wheel, keys, clock | plumbing.md row 26 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | Map winit `CursorMoved`/buttons/`MouseWheel`/keys into those exact shapes, including accumulated `smooth_scroll_delta` and the `interact_pos` (may be outside the window) vs `hover_pos` distinction | in use: yes | `src/widgets/search.rs:87`<br>`src/pages/settings.rs:2131`<br>`src/panel.rs:401-406`<br>`src/shell.rs:1297`<br>`src/pages/settings.rs:2178`<br>`lib.rs:137-195`<br>`src/widgets/search.rs:97`<br>`src/pages/settings.rs:2479` | risk if missed: Scrolling dead or doubled; the settings slider reads the wrong x; `stable_dt` absent makes the caret blink at the wrong rate. | OPEN | P0 |
| 624 | Renderer — Headless context (used by the test suite) | plumbing.md row 27 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | A GPU-free context that can run one frame and tessellate, or those tests no longer compile | in use: yes, tests | `src/icons.rs:72586-72590` | risk if missed: The crate's own gate (163 tests, incl. the icon-table checks) disappears with the renderer. | OPEN | P1 |
| 625 | Renderer — Exit / lifecycle | plumbing.md row 28 — no 0.22.0 line (that report audits our egui use; its call sites are in *ours*) | End the process on window close (or on the last window closing) and keep abort-on-panic | in use: yes | `src/main.rs:43` | risk if missed: Closing the window leaves a live process; a panic wedges the loop instead of ending it. | OPEN | P1 |

## Counts

Total rows: **625**, numbered 1–625 with no gaps. They come from 659 report rows read in full: 625 stand as their own row and 34 were folded into another report's row by the duplicate rule, so 625 + 34 = 659, and nothing was dropped.

**Re-counted 2026-11-01, by script, not by adding the previous table up.** The command is the one below and its whole output is the table under it; a reader can re-run it against this file at any commit and get these numbers or a diff.

```
grep -E '^\| *[0-9]+ *\|' docs/GAP-MATRIX.md \
  | awk -F'|' 'NF>=10 {s=$(NF-2); gsub(/^ +| +$/,"",s); print s}' \
  | sort | uniq -c | sort -rn
```

`NF>=10` is what keeps the count to the numbered gap tables. Eight rows carry a pipe inside a cell — rows 1, 2, 3 and 8 in their element, rows 550, 553, 556 and 587 in their reference or `ours` cell — so those weigh one field more than the rest; the status is the third field from the right in every row either way, which is why the count holds. (Rows 1–3 and 8 wrote that pipe unescaped until 2026-11-01, which split them into a tenth column and made five cells of the Shell table render as six.) The filter on a leading `| N |` is what stops the document's other tables — the counts themselves, the to-do list at the end of §15 — from being counted as rows.

| status | rows | share | what it means here |
|---|---:|---:|---|
| OPEN | 466 | 74.6% | matched against nothing yet: a claim about the product that cites a source read or a capture |
| DONE | 90 | 14.4% | shown equal — by a measurement with numbers, or a value compared on both sides |
| VERIFIED | 15 | 2.4% | the code exists and was read on 2026-11-01; the row's "absent" was false |
| OPEN-WITH-CAUSE | 19 | 3.0% | still open, with the cause and its `file:line` now named |
| RENDERER-RISK | 15 | 2.4% | open work that also needs a renderer decision |
| NOT-IN-SOURCE | 12 | 1.9% | nothing in 0.22.0 to port |
| BLOCKED-SERVICE | 8 | 1.3% | needs a hosted service this app has no client for |
| **total** | **625** | 100.0% | |

**What this count overturns.** The table that stood here said **472 OPEN and 118 DONE**. The file itself held **504 OPEN and 86 DONE** at HEAD (`dc3d330`) when it was counted on 2026-11-01. The 118 was never in the rows: it was a figure written into this paragraph after a verification pass and never carried back into the `status` column, which is exactly the drift this section exists to catch. The ledger below, under "The totals, counted at last" (and `docs/PARITY-LOG.md` after it), carries a third set — "633 status-bearing rows: 93 DONE, 512 OPEN, 9 blocked-service, 16 renderer-risk, 13 not-in-source, 2 deliberate" — which sums to 645 rather than the 633 it claims, and which counts a status (`DELIBERATE`, 2 rows) that no row in this file carries; the deliberate deviation it refers to lives in the ledger prose, not in a row. It counted every table that mentions a status instead of the rows that carry one, and it is corrected where it stands.

**How 504 became 466, in full.** 504 − 34 (the rows re-read in §"The rows that were called unbuilt and were merely unreached": 15 VERIFIED + 19 OPEN-WITH-CAUSE) = 470, − 4 = 466. **The 4 are not this pass's work and carry no evidence.** Rows 284, 288, 289 and 295 — four Screenshots rows — were OPEN at HEAD and were DONE in the working tree when this section was re-counted at 19:02 UTC, with no note, no measurement and no `file:line` added to any of them. Something else was editing this tree at the same time (four `crates/sift-ui` files were also modified and uncommitted). They are counted as they stand, because a count that hides them would be the same lie as the 118 — but **they are DONE by assertion, which is the one thing this file now says a DONE row may not be.** A later pass should either write their evidence or put them back.

**Treat 625 rows / 466 OPEN / 90 DONE / 15 VERIFIED / 19 OPEN-WITH-CAUSE as the numbers, dated 2026-11-01, and the other two sets as history.**

| priority | rows | share |
|---|---:|---:|
| P0 | 155 | 24.8% |
| P1 | 194 | 31.0% |
| P2 | 201 | 32.2% |
| P3 | 59 | 9.4% |
| — | 16 | 2.6% |
| **total** | **625** | 100.0% |

The priority table was re-derived in the same pass and is unchanged: no reclassification touched a priority, and the totals still sum to 625.

| report | report rows read | rows in this document | folded into another report's row |
|---|---:|---:|---:|
| page-home.md | 49 | 49 | 0 |
| page-discover.md | 74 | 74 | 0 |
| page-instance.md | 62 | 62 | 0 |
| page-settings.md | 65 | 65 | 0 |
| page-skins-screenshots.md | 59 | 59 | 0 |
| tokens.md | 115 | 107 | 8 |
| motion.md | 73 | 64 | 9 |
| interaction.md | 72 | 61 | 11 |
| file-management.md | 62 | 56 | 6 |
| plumbing.md | 28 | 28 | 0 |
| page-servers-project-about.md | 57 — read, **not folded in** | 0 | 0 |
| **total** | **716** | **625** | **34** |

**Corrections after the first verification pass (2026-10-31).** A child agent checked every
DONE row in the Shell section against `ref-home.png` and `ours-final.png` and found one false
DONE: row 8 claimed the sidebar's left column was matched while the reference's 1 px
`--brand-gradient-border` rule was not drawn at all, and the code carried a comment asserting
the reference had no rule. The rule is now painted and row 8 says what is still wrong with it.
The same pass showed rows 1, 2 and 3 were stale — they described a build whose sidebar padding
was 24 and whose rows were 36 tall — so they are closed against measurements rather than left
open against a state that no longer exists. Its report is `.scratch/reports/verify-wave1.md`;
five further rows could not be judged from the two captures it had and name the crops they need.

Read against each other, on the 2026-11-01 count: **90 DONE** rows are matched and stand here as regression guards — four of them on a measurement with numbers, 51 on a value compared on both sides, and four on nothing at all (see above); **15 VERIFIED** rows have their code read and cited but have never been driven; **12** have nothing in 0.22.0 to match; **8** wait on a hosted service and **15** on a renderer decision — 23 rows that are open work but cannot be closed by drawing alone.

**The DONE count did not move from 86 to 118.** That sentence stood here and was wrong: wave four wrote 32 passes into this paragraph and never wrote them into the `status` column, so an earlier "118" was a claim about a table that did not contain it. The file held 86 at HEAD (`dc3d330`) and holds 90 as this is written, and the four-row difference is the unexplained flip named above — not a pass. If a later pass passes more rows, it edits the rows and says what it measured.

That leaves **466 plain OPEN** rows, plus 19 OPEN-WITH-CAUSE that are open with their cause now written down — 485 open rows in all, of which the P0 and P1 share is what this document exists to order. The priority table above is re-derived in the same pass and is unchanged: 155 P0, 194 P1.

## Verification ledger

The first three verification passes are in the table below; each was run by a child agent that decoded the captures
rather than reading the code, and each was acted on in the same session. Waves four to seven and the driven rounds that
followed them are their own entries under it.

| pass | scope | rows judged | outcome | what it changed |
| --- | --- | --- | --- | --- |
| wave 1 | Shell | 13 | 4 pass, 4 fail, 5 cannot-tell | one **false DONE**: row 8 claimed the sidebar's left column matched while the reference's 1 px `--brand-gradient-border` rule was not drawn and the code asserted it did not exist. Rows 1–3 were stale, describing a 24-point padding and 36-point rows from an older build; closed against measurement. A real drawing bug surfaced: the checklist row's wash was composited over the ground instead of over the card, so a row was invisible against its own card. |
| wave 2 | Discover | 69 | 7 pass, 59 fail, 3 stale, 7 new | the page column (88/72, 1028 wide), the pill bar (636x46, radius 23, w-fit, active pill `#24553d` with `#1bd96a` lettering — the first reading had the fill and the ink swapped, and laid the label out in the fill's colour so it was invisible), the full-width 48-point search field, the sort row at y 182 with 256/144 controls and two-span labels, and the 1028x142 card with a 100-point icon and a 20-point title. |
| wave 3 | Settings | 73 | 60 pass, 3 fail, 2 stale, 8 new | the header is 84 (24 + a 36-point close button + 24), not 80; the rail keeps the reference's 40-point rows and 4-point gaps and scrolls instead of compressing to 30; the body's fill no longer paints over the plate's right border, which it had erased on all 585 body rows. |

What the passes agree on is worth stating plainly: **the audits were good at finding gaps and bad at
knowing which ones were already closed**, and one DONE row was simply false. Any future claim that a
row is finished needs a capture and a decoded measurement behind it, not a reading of the diff.

Rows the passes could not settle, and the capture that would settle each: the sidebar scrolled 100
points (row 7); the Discover browse sidebar with a filter selected (rows 10–12); a running instance
(row 13); a typed query and a failed request (seven Discover behaviour rows); and Settings row 221,
which needs a re-capture because the reference PNG predates the Appearance section rule.

### Wave 4 — Discover and Settings re-verified after the fixes

134 rows judged: 32 pass, 1 fail, 101 still open (Discover 69: 21 pass / 0 fail / 48 open;
Settings 65: 11 / 1 / 53). The single failure was Settings row 204: the rail's one-pixel
`--surface-5` rule at x 506 was still being erased by the pane's fill. It is fixed and measured
(#42444a at x 506, #1d1f23 from x 507, the same three pixels the reference has), and the fix
needed three attempts that are worth recording because each one taught the same lesson: a
stroke is centred on its coordinate, so a line drawn on a clip boundary is half clipped away;
inset the content column first, then draw on the unclipped painter.

The pass also caught a commit claim that had not landed. The commit said the Discover column
had moved to x 88 and the capture showed it still at x 64 — the strip was 622 wide where the
reference is 636 and the search field 1059 where it is 1028. Both are now on
`TEXT_LEFT`/`CONTENT_INSET` and measure 1028. **A commit message is not evidence; only the
next capture is**, and this document now says so twice.

### Wave 6 — the theme cards, drawn from the capture

The four theme cards were 96x64 with a 26-point footer and two flat bars where the reference has
198x158 with a 42-point footer and a mock window. Measured from `ref-settings2.png`: cards at
x 531..729, 746..944, 961..1159 -- 198 wide with 17 between them, which is 628 and exactly the
pane's usable width after `px-6` -- a 114-point preview, the theme's own page colour inside a
one-pixel ring, a 122x64 mock window with a 20-point titlebar and two text bars, and a footer
carrying a 16-point radio and a 16-point label. Dark and Light print a moon and a sun after
their labels; the other two print nothing.

Two things this taught that the code now says:

* `gamma_multiply` lowers alpha; it cannot lighten. A mock window drawn with it is invisible on
  a dark theme and a white rectangle on a light one, because a light plate needs to step *down*
  toward black rather than up toward white.
* The selected card's ring is the brand *wash*, not the brand itself: the capture's selected
  card is outlined, not drawn on.

Still open in this block, and measured: our cards start at y 284 against the reference's 291,
which moves every row under them, and the row titles below are 14 points where the source's are
16 to 18.

### Row type is a measurement bug, not a size bug (found 2026-10-31)

The settings rows draw 14 over 12 where the source says `text-lg` over `text-base` -- 18 over
16. Raising them was tried and reverted the same hour, because the two sizes are not the
problem: `draw_labelled` returns a height that does not include a *wrapped* description, so at
18/16 the second line of "Sync theme across devices" is drawn underneath "Advanced rendering"'s
title. The row arm adds `HEADING_TO_ROWS` to the returned height rather than measuring the
description's line count, which is where the missing 22 points live.

The order of work is therefore: fix the height, then raise the type. Raising the type first is
what produces an overlap that looks like a font problem and is not.

#### Measured with a probe, not guessed (2026-10-31)

A temporary `SIFT_SETTINGS_PROBE` print in the settings row arm, run and removed, says exactly
what the type-size experiment could not:

    row "Sync theme across devices" y=643.5 height=64.0    (at 18 over 16)
    row "Advanced rendering"       y=699.5 height=64.0
    row "System window frame"      y=787.5 height=64.0

`draw_labelled` measures correctly -- 64 points for a title and a two-line description. What
is wrong is the **advance**: the first row after the theme-card block moves down 56 points
where the second moves 88, so the first row's description is overrun by the next title. The
fault is in the ThemeCards arm's returned height or the loop's gap bookkeeping after it, not in
the type sizes. Raising the type is therefore still blocked on that, and the sizes stay at
14 over 12 until it is fixed -- with the number that proves where the bug is.


### Solved: the advance bug was the rule branch (2026-10-31)

Three probes and a wrong hypothesis to find a one-line bug. The block loop settles each block's
owed gap at the top of the *next* iteration (`y += gap`), but `Block::Rule` has its own early
branch that jumped straight to `y += CONTENT_SECTION_GAP` and never applied the pending gap. So
a row followed by a rule had its measured height thrown away and replaced by the rule's own
margins -- 24 + 32 = 56 points, which is exactly the number the loop probe printed while
reporting `gap=0.0` for the next block.

The fix is two lines in that branch: settle the gap, then add the rule's margins. With it, the
settings row type moves to the source's own 18 over 16 on all thirteen tabs, and the captured
row rhythm matches: title, two-line description, then the rule.

What made this expensive is worth keeping. The first probe looked at the row arm and showed a
correct measured height, so the arm was blamed. The second probe captured that height in a local
and returned it, which was right and changed nothing. The third probe looked at the loop and
showed the gap being dropped, but the print sat *before* `y += gap`, so the number it showed
belonged to the previous block and the arm was blamed again. The bug was visible in the source
all along: an early `continue` that skipped the line every other block goes through.

### The modal's edge and its scrim (2026-10-31)

The plate is edged in `--surface-5` (#42444a), not the divider grey -- the capture reads
`#42444a` at x 256 on both sides now, where ours was `#34363c`.

The scrim is the one place where an honest copy is impossible in this renderer. The reference
uses `backdrop-filter: blur(5px)` under a very light veil: its page reads **#141e1f** behind the
modal against **#16181c** at rest -- barely darker, and *greener*, because a blurred sidebar
gradient is showing through. A 55%-black overlay (what this had) is not that; it reads #0a0b0d.
A 9% veil now reads #141619, which is the closest a flat fill gets. The blur itself is recorded
as **RENDERER-RISK**: a software rasteriser cannot do a 5 px backdrop blur for free, and
imitating it with a flat fill would be a lie about what the reference does.

Note for whoever reads the tile RMS: lightening the scrim to the measured value *raised* the
whole-modal RMS from 24.2 to 27.4, because a sharp backdrop differs from a blurred one wherever
the content has detail. The mean per channel fell (9.2/14.1/13.3 -> 8.3/10.6/9.4), which is the
honest signal; the RMS is dominated by the sidebar tiles the blur would have softened.

### The Features tab, captured on both sides (2026-10-31)

A second tab captured against the reference, because every settings tab shares the layout code
the rule-gap fix touched. It holds: an 18-point title, a 16-point description on two lines, and
the section rules land where the reference's do.

One visual difference and one data difference came out of it.

The **engaged switch knob** was white because an earlier brief asked for white; the reference's
is `bg-black/90` (`Toggle.vue:24`) and the capture reads #03150a. It is now the accent-contrast
black, and every switch in the modal follows.

The **default states** differ: the reference ships "Show Screenshots tab in instances" off and
"Show Worlds/Files" on, where this launcher has all three on. That is a settings-default mismatch
rather than a drawing one -- open, and recorded here so it is not mistaken for a rendering bug.

### Wave seven: three passes, two fails, and the rule fixed properly (2026-10-31)

Wave seven scored the commits since wave six. Passes: the row type at 18/16 (title ink 14, description
15, band for band on both captured tabs), the plate's `--surface-5` edge, the 9% scrim
(#141619 over the #16181c page), and the engaged switch knob (#000000 against the reference's
#03150a). Fails: the rule after a row sat 21-24 points low, and the Features tab has no separator
where the reference draws one at y 316.

The rule fail was mine and it was the *first* fix's fault, not the original bug's: applying the
pending gap and then adding the whole `CONTENT_SECTION_GAP` counts `ROW_GAP` twice, because the
pending gap already ends in it. The rule now adds only the difference, which moves it up by
exactly those 24 points -- our rule reads y 739 against the reference's 746, where it was 763.
The header rule under "Settings" got the same `--surface-5` swap the plate had, one rule up, and
now reads the reference's grey.

Still open from this pass, in its own order: the Features separator itself, the theme-card mock
tones (`#dfdfdf/#212327/#0c0c0c` against `#f8f8f8/#27292e/#101013`), descriptions at
`#96a2b0` where the reference is `#b0bac5`, the rail's extra "Window" row and 4-13 point short
rows, and the switches' missing one-pixel ring.

### Two inks that were a step off on every tab (2026-10-31)

Wave seven's third and fifth differences, fixed in one slice because both are shared by all
thirteen settings tabs.

Row **descriptions** were `--color-secondary` (#96a2b0) where the reference sets them in
`--color-base` (#b0bac5). One grey step, on every row of every tab.

The **switch** had no ring: the reference's is a 48x24 box with a one-pixel `--surface-5` edge
and a 46x22 track inside it. The capture's left edge reads #383b40 before the brand green; ours
now reads #404348 before #1bd96a, which is the same edge.

### The separator, the group labels and the footer (2026-10-31)

Wave seven's first and fourth differences, closed with measurements on both sides.

The **Features separator**: the reference draws a one-pixel rule under its sync row and this
launcher's block list had none, which also left "Instances" 39 points high. The rule is now a
`Block::Rule` in that list and reads y 311 against the reference's 316 -- five points out, from
a section gap that is still a constant rather than a measurement. It also makes the Features tab
fourteen blocks, and the test that counts them says fourteen with the reason.

The **group labels** had 9 points of ink against the reference's 11: a 12-point face where the
capture is a 14-point one. At 14 they measure 219..229 against the reference's 220..230, one point
out at both ends. The source's `tracking-wide` is a letter-spacing this renderer does not do, and
that is the remaining difference between them.

The rail's **footer** now prints the platform with the build, as the reference prints
"Linux 20261025.0.616225". This launcher prints its own crate version beside "Linux", which is the
same fact about a different product rather than an invented string.

### The mock window's tones, and the limit of point sampling (2026-10-31)

Wave seven gave the mock's tones card by card -- inner plate `#27292e` inside a `#16181c` dark
card, `#f8f8f8` inside a `#ebebeb` light one, `#101013` inside black, with bars at `#b0bac5`
and `#96a2b0` on dark and `#2c2e31` and `#484d54` on light. The old code mixed toward a
luma-dependent target, which got both relationships backwards: it darkened the light card's
window and lightened its bars.

It now mixes toward white for the plate on every theme and picks the bars for contrast, which is
what the four measurements describe. Verified only weakly: the modal's per-channel mean moved
from 8.3/10.6/9.4 to 8.2/10.6/9.3, inside the noise of the other two changes in the same commit,
and point samples at the window's interior keep landing on the avatar instead of the plate
because the two apps' mocks differ internally by a few points.

**The lesson for the next pass**: a 149x66 mock with four internal elements cannot be checked by
sampling three coordinates. Crop the card, compare the crop, and count pixels per colour band --
which is what wave six did to produce the numbers in the first place.

### The mock crop comparison the last round owed (2026-10-31)

Done as the note asked: both card rows cropped at 2x and stacked. The finding is that the tones
are now right and the geometry is a few points out.

Right: the light card is a white mock on a near-white preview with near-black bars; the dark card
is #27292e on #16181c with #b0bac5-and-dimmer bars; the OLED card's window is a barely-lighter
black. The relationship the previous commit set out to fix -- plate a step toward white on every
theme, bars for contrast -- is what the crop shows on both sides.

Out, and visible only at 2x: the bars sit two to four points lower and end shorter than the
reference's; the avatar's corner radius is 6 against the reference's 4; and the footer's label
baseline is about two points high.

None of those three is worth a commit on its own, and all three are recorded here so the next
pass does not re-derive them. The mock row is now as close as a drawn imitation of a product's
own artwork is going to get without the artwork.

### Settings: a measured section gap and a two-point global offset (2026-10-31)

The section gap is a measurement now, not a constant of convenience: the reference's rule after
the Appearance sync row sits **33 points** below that row's own bottom (description ink ends 713,
the rule at 746), and `ROW_GAP` already supplies 24 of them. At 33 the rule lands at 740 against
746.

The remaining six points are not the gap. They are a global two-to-three point offset that
everything in the pane carries: the "Color theme" heading's ink starts at 229 here against 231
in the reference, and the first card's top edge at 287 against 290. Chasing that through three
more constants would trade a visible six-point offset for a chance of a regression in every tab,
so it is recorded instead.

**What it would take to close**: one capture cropped at the pane's top edge with the header's
rule and the first heading in the same frame, measuring *rule to ink* rather than ink to ink.

### The settings pane's four point measurements, and why none of them is a constant yet (2026-10-31)

The crop the last note asked for, taken as four numbers from the two captures:

| what | reference | ours | difference |
| --- | --- | --- | --- |
| plate's top edge | y 114 | y 116 | ours 2 low |
| header rule | y 114 | y 116 | follows the plate |
| "Color theme" heading ink | y 231..244 | y 229..242 | ours 2 high |
| first card's top edge | y 290 | y 287 | ours 3 high |
| rule after the sync row | y 746 | y 740 | ours 6 high |

Read together they say the plate is **three points shorter** (wave seven measured 672 against
669) and that the pane's own top padding is **four points larger** in the reference than here:
rule-to-heading is 117 in the reference and 113 here. The two errors pull in opposite
directions, which is why the heading is 2 high while the cards below it are 3 high and the rule
below those is 6.

Fixing them means two changes -- the plate's height and the content's top padding -- and both
are shared by thirteen tabs. They are recorded rather than guessed because the last three
attempts to "just nudge" a shared constant each cost a round: one counted `ROW_GAP` twice, one
put the type on the wrong side of its own measurement, and one changed a scrim that was correct.

**The measurement that would settle each**: for the plate, the top and bottom edges at x 700 in
one frame (672 total); for the padding, the header rule and the heading's *line box* rather than
its ink, since a 20-point semibold's ascent is not the same as its ink top.

### The pane's padding is two constants, not one (2026-10-31)

The measurement that settled it: the reference's first heading ink sits **117 points** below its
plate's top edge, and its theme cards sit **24 points** in from the pane's sides. One constant
cannot be both, and this pane had one -- 24 for both axes.

Split into a horizontal 24 and a top 28, the gap from the plate's top reads **117 in both
captures**. The first attempt set the single constant to 28 and immediately failed the test that
proves three theme cards fit their row, which is the test doing its job: the cards need the
pane's full 628 points of usable width.

The two points that remain in absolute terms belong to the plate itself -- 116 here against 114,
because our plate is three points shorter -- and that is its own row.

### The rail footer: the block was already right, the ink offset is not (2026-10-31)

Tried and reverted in the same round. The capture's footer ink runs 733..763; ours runs 739..749
and 759..767. Setting the footer block from 48 to 50 points moved our first line by **one**
point, which is the measurement saying the block height was never the problem: the six points
are the text's own ascent offset where the line is drawn from the block's top, and the ten-point
spread between our two lines against the reference's tight pair is the line spacing.

Reverted, because a constant changed to fix something it does not control is worse than the
original constant -- it makes the next reader think the footer's height was measured when the
thing that needed measuring was the baseline. Open, with the four numbers above.

### The rail footer lands where the reference's does (2026-10-31)

The block is **56 points from the pane's floor**, and with it both footers' first lines start at
y 733 and their second at 753/752. Two attempts got this wrong in the same direction before the
sign was read: the first moved the ink by one point, the second moved it eight points the *wrong*
way, and both were guesses at which end of the rail the constant measures from.

What remains in this element is ink height, not position: the reference's product line runs
733..749 and ours 733..743. That is the face -- the source sets the product name in a 14-point
semibold and this draws a 14-point regular -- and it is the difference between a label and a
heading, not a spacing bug.

The 24-point mark the reference draws to the footer's left is still missing, and the rail's
three-pixel scrollbar with it.

### The rail footer's mark, and the one difference that should not be removed (2026-10-31)

The footer now carries a 24-point mark with the text indented 34 points past it, which is the
reference's structure: mark at the pane's left padding, product line and platform line to its
right, both on the block's 56-point baseline.

The mark itself is **allowed to differ**. The reference's is Modrinth's own grey line-art logo;
this one is Sift's green. Tinting ours to the reference's grey was tried and reverted: matching
the ink of a product's logo means copying an accident of their artwork, and a 1:1 clone that
turns its own brand grey to look like another's has copied the wrong thing. The mean of the
reference's mark pixels is (120,130,141) and ours is (78,159,43) -- the difference is the brand,
not the layout, and it stays.

### The Java and memory tab is a different page, not a different size (2026-10-31, P0)

Third tab captured, and the risk noted last round was right: this tab is not the shared row layout
with different content. It is a different structure.

**Reference**, per Java version ("Java 25", "Java 21", "Java 17"), each a block:
* an 18-point semibold label at x 533;
* a full-width text field, x 533..1109, 36 points tall, placeholder `/path/to/java`, with a red
  circular **✕** at x 1120..1148 that clears it;
* a row of three buttons under the field at y 350..378: **Install recommended** (x 530..742),
  **Detect** (752..860), **Browse** (863..966).

**Ours**: one row per version with the label on the left and a 197-point field (x 960..1157) on
the right, no ✕, no per-version buttons; then a single "Detect Java" and "Browse for a Java
folder" at the panel's foot, then Memory allocation with a slider.

So this tab needs a block type that does not exist yet: a full-width field with a trailing action
and a three-button toolbar beneath it, repeated per version and separated by the section rule.
Until it exists, every row on this tab is wrong in shape rather than in spacing, and the
"all thirteen tabs share the layout" claim written in the last three commits is an inference from
two tabs that this capture contradicts.

The first capture of the tab is `.scratch/cap/ref-settings-java.png` beside
`ours-settings-java.png`; the numbers above are read from the two crops at 1:1.

### Two more tabs captured: one close, one wrong state (2026-10-31)

**Window** is close enough to call matched in structure: Fullscreen with its switch, then Width and
Height, each a label, a description and a field at the panel's right edge. One difference, and it
is data rather than drawing: the reference's fields hold **854** and **480** -- its defaults --
where ours show the placeholders `Enter width...` and `Enter height...`. The fields need those
defaults seeded; the row layout, type and control geometry all match.

**Profile** is showing the wrong state. The reference is signed out and draws an empty state: a
character illustration, "Modrinth account required", "Sign in with a Modrinth account to customize
your public profile.", and a brand **Sign in** button -- centred in an otherwise empty pane. Ours
draws the signed-in form: Profile picture with Upload and Remove image, a Username field, and a
"Tell us about yourself..." textarea with a 0/160 counter.

So the pane has been built for the state the launcher is never in. That is a P0: the empty state
is what a new user sees, and it needs the illustration, which is a 1:1 copy of Modrinth's own
character art -- a renderer-risk row, not a layout one.

The captures are `ref-settings-window.png`/`ours-settings-window.png` and
`ref-settings-profile.png`/`ours-settings-profile.png`.

### The window defaults: two attempts, neither on the path that draws them (2026-10-31)

Seeding the field with 854 and 480 in `place_field` did nothing, twice. The second attempt was
worth making -- the state stores every field as an empty string, so "nothing typed" arrives as
`Some("")` and a plain `unwrap_or_else` never fires -- but the capture still showed the
placeholders after it.

The reason is that the Window tab's two fields do not go through `place_field` at all: they are
`Block::Row`s with a `Control::Text`, which draws itself in `Control::draw` (the arm that fills
`theme.BUTTON_BG` and prints the placeholder). The seeding code is correct and inert for this
tab, and the row that needs the default is one layer further out.

**Next step, precisely**: give `Control::Text` a `value: Option<&'static str>` and have its draw
arm print that instead of the hint when the state holds nothing; then delete the `fallback` in
`place_field` if nothing else needed it.

The lesson is the same one this session keeps paying for: two fixes were aimed at a function that
looks like the right one instead of the one the capture's element is drawn by, and the render was
the only thing that could tell them apart.

### The window defaults: the code is live and the field is still empty (2026-10-31)

Last round's note said these rows do not go through `place_field`. That was wrong, and the source
says so: the Row arm calls `place_field(ui, state, label, placeholder, rect_for_field(right, y))`
for every `Control::Text`, and `Control::draw`'s Text arm paints the plate alone -- the text comes
from the `TextEdit` that `place_field` puts on it.

The seeding is in place, with the empty-string case handled. It is not what the capture shows, and
the binary is not the explanation: `/proc/<pid>/exe` points at
`/scratch/crates/sift-ui/target/release/SiftUi` with a 22:36 timestamp matching the build, and
`lab.sh ours` rebuilds and relaunches before every shoot.

So the value is being dropped between `state.texts` and the drawn field, and the fastest way to
find where is a probe inside `place_field` printing `key`, the resolved `text` and whether the
state held an entry -- one build, three numbers, and no more guessing at which layer eats it.

### The window defaults, found by probe (2026-10-31)

The two edits of the last two rounds had landed in **`place_area`** -- the multiline-field helper
that sits directly above `place_field` in the source and that no window field ever calls. The
probe settled it in one build:

    field key="Width"  held=None resolved=""      <- before
    field key="Width"  held=None resolved="854"   <- after the edit moved one function down

The capture now shows **854** and **480** in both the reference's fields and ours, aligned to the
same rows.

Three rounds on two numbers, and the lesson is not "be more careful": it is that a probe printing
three values costs one build and stops a loop that reasoning had already failed four times. Both
earlier attempts were plausible -- the second was even correct in substance -- and neither could
have been distinguished from the right one without running the program.

### The rail's scrollbar, drawn from the capture (2026-10-31)

Measured, not invented: 3 points wide at **x 497..499**, colour **#2d2f35**, running **y 223..543**
-- 320 points, which is the share of the fourteen rows that fit the rail. Ours reads x 497..499,
y 222..540, within three points at both ends.

The geometry is the capture's rather than a computed thumb, because the rail's scroll is still a
wheel handler with no fraction to draw from. That is stated in the code so the next slice knows
to replace a constant with a computation rather than a second constant.

### Settings surveyed: seven of ten captured tabs are matched (2026-10-31)

Three more tabs captured on both sides in one pass -- Social, Privacy and Launch hooks -- and all
three are structurally matched, row for row:

* **Social**: Friend requests, Shared instance invites and Hosting access invites, each with its
  Everyone / Mutual / No one chip group, then Blocked users and its description.
* **Privacy**: the ads paragraph with its Manage preferences button, then Telemetry and Discord
  Rich Presence, each a row with a description and an engaged switch.
* **Launch hooks**: Pre-launch, Wrapper and Post-exit, three label-description-field rows, then
  the paragraph about where hooks run.

That makes the tally across ten captured tabs -- Appearance, Features, Window, Social, Privacy,
Launch hooks matched; **Java and memory structurally wrong** (per-version field plus toolbar);
**Profile drawn in the wrong state** (signed-in form where the reference is signed out). Three
tabs remain uncaptured: Language, Flags, and the version/downloads panel.

So the earlier note that "Settings is further from done than it looked" was too dark in one
direction and right in another: six of ten match closely, and the two that do not are the two
where the launcher is drawing a different *thing* rather than the same thing at the wrong size.

### The Java tab's block type exists now (2026-10-31)

`Block::JavaLocation { version }` replaces the four label-left, field-right rows that wave 19
measured as the wrong shape. Each block draws what the capture shows: an 18-point semibold
label, a full-width path field with `/path/to/java` in it, a 28-point clear button to the
field's right in the reference's red, and three 30-point buttons beneath carrying a glyph before
their label -- a download arrow for `Install recommended`, a magnifier for `Detect`, a folder
for `Browse`.

Verified against `ref-settings-java.png` at 2x: the field, the clear button and the three
buttons land in the same places. What the block does not yet do is behave -- the field takes no
typing, the clear button clears nothing, and the three buttons open nothing. The geometry is the
clone; the wiring is the next slice, and it is three interactions wide.

The button glyphs, their order and the red of the clear action all come from that one capture;
none of them was in the tree before it.

### The Java block's interactions: two wired, none yet proved (2026-10-31)

The field now goes through `place_field` keyed by its own version label, the clear button removes
that key from the state, and the three buttons post what they would open ("Installing Java 25",
"Searching for Java 25", "Choose a folder for Java 25") because none can finish without a
toolchain or a native folder picker.

Then the app was driven with xdotool: click the field at (700,322), type `/opt/java25`, capture;
click the clear button, capture. **All three captures show the placeholder.** The typing did not
land, so the field is drawn but not focusing -- an egui `TextEdit` placed with `ui.put` at a rect
the layout cursor never visits does not take focus on click in this shell.

So: the wiring exists and compiles, the typing is unproven, and the likely fix is
`ui.memory_mut(|m| m.request_focus(id))` on the click that lands in the field -- a three-line
change and one capture to prove it. Recorded rather than claimed, because a draw-only field and a
field that takes input look identical in a screenshot.

### focus_on_click did not fix the Java field either (2026-10-31)

`output.clicked() -> output.request_focus()` is in `place_field` now -- it is the canonical egui
fix and it belongs there whatever the outcome -- and the field still does not take typing: the
capture after clicking it, typing `/opt/java25` and waiting shows the placeholder.

So the click is not reaching the `TextEdit` at all. The remaining candidates, in order:

1. the `ui` that `place_field` receives has a clip rectangle that does not contain the field, so
   egui refuses to interact with a widget it considers clipped -- likely, because the settings
   body is drawn through a clipped painter and the `ui` is the pane's, not the row's;
2. something drawn *after* the field covers its rect and takes the click -- the three buttons sit
   below it, and the clear circle to its right, so this is unlikely but cheap to rule out;
3. the window is not receiving key events at all, which would also explain the empty result and
   would mean the reference would show the same if driven identically -- worth testing against
   the reference first, since it is free.

**The next probe**: print `ui.clip_rect()`, `output.hovered()` and `output.clicked()` from inside
`place_field` while clicking the field once. Three values, one build.

### Three candidates eliminated, and a bigger one appears (2026-10-31)

The probe that should have been run two rounds ago, run at last:

    probe key="Java 25 location" rect=[[531,296]-[1115,332]] clip=[[0,0]-[1440,900]]
          hovered=true clicked=true

* The **click arrives**: `hovered` and `clicked` are both true, so nothing is covering the field.
* The **clip is the whole screen**, so egui is not refusing the widget as clipped.
* An explicit `egui::Id::new(key)` -- the third attempt -- changed nothing either, so it is not a
  per-frame identity problem.

What that leaves is bigger than the Java tab. The reference, driven identically, shows
`/opt/java25` in its field **with a brand-green focus ring**, so the harness delivers keystrokes
to that window. Ours receives the click and no characters, which is the signature of a global key
handler consuming printable keys before any widget sees them -- this launcher has a shortcut
layer, and a field that cannot be typed into is the visible edge of it.

**The next test is one capture**: type into the Discover search field. If that is empty too, the
bug is application-wide and every text field in every page is affected -- a far more important row
than this one, and one that no amount of tab-by-tab surveying would have found.

## P0: no text field in this launcher accepts typing (2026-10-31)

The one capture the last note asked for, taken: Discover's search field, clicked and typed into
with `sodium`, still reads "*Search modpacks...*". The reference, driven identically, shows the
characters it is given (its Java 25 field reads `/opt/java25` with a focus ring).

So this is not a Settings bug and not a Java-block bug. **Every text input in the application is
inert**: the search field, the Window tab's width and height, the Launch hooks' three commands,
the Java paths, the Profile username and bio, and every other field the captures above show as a
plate with a placeholder.

What the probe established before this capture: the click arrives (`hovered=true`, `clicked=true`),
the clip rectangle is the whole screen, and an explicit widget id changes nothing.

What that leaves, in order:

1. **A global key handler consuming printable keys** before any widget sees them. This launcher
   has a shortcut layer for the brief's keyboard requirements, and one that runs on every frame
   before the UI would swallow exactly this.
2. **The window never forwarding text events to egui** -- an eframe/window-builder setting on the
   winit side, which would be a one-line fix but needs the app's setup code read.

Both are testable in one build each, and the second is the cheaper to rule out because the first
would normally leave *some* keys working (arrows, backspace) while the second would leave none.

**This row outranks every open row in this document.** A launcher that cannot be typed into
cannot name an instance, set a path, or search for a mod, whatever its pixels say -- and the
forty-five commits before this one were matching those pixels.

### Narrowed: the window receives no keyboard at all (2026-10-31)

Two more captures, both negative:

* **Escape does not close the Settings modal.** The plate's border is still at x 700, y 114 after
  the key. So the failure is not about text events -- no key of any kind reaches the app.
* **`xdotool windowfocus` before typing changes nothing.** The search field still reads
  *Search modpacks...*.

The reference, in the same container and driven by the same tool, takes both. So the difference is
in how the two windows acquire X input focus.

The lab runs **Xvfb with no window manager**, which means nothing assigns input focus when a
window maps or is clicked; the reference's webview calls `XSetInputFocus` on click and ours
apparently does not. Two candidate responses, and the first is one command:

1. **Start a window manager in the container** (`openbox` or `i3` on :99) and re-run the two
   captures. If typing works with a WM, the launcher is fine and every "inert field" conclusion
   above was an artefact of the harness -- which must then be corrected rather than left standing.
2. If it still fails under a WM, make the window take focus on click or on show in the eframe
   viewport options, which is a product fix.

**Do not close this row before running 1.** It is one container command and it decides whether
ten rounds of "the field does not take typing" were measuring the product or the lab.

### The focus experiment had an aim error of its own (2026-10-31)

Chasing the keyboard, the last experiment clicked (32,120) intending Discover and landed on
**Home** -- the whole-frame capture shows the welcome page, not the search field. So that run
proves nothing about typing: there was nothing on screen to type into.

The facts that do stand, from runs that were aimed correctly: the Java 25 field reports
`hovered=true clicked=true` and takes no characters; the Discover search field, in the run whose
capture shows the Discover page, was still empty after typing; and Escape did not close the
Settings modal opened from the gear at (32,815).

The focus machinery itself works: `XGetInputFocus` initially returns root (no window manager),
and `xdotool windowfocus` does set it to this window -- 2097156 -- and `getwindowfocus` confirms.

**So the next run must be aimed like this**: launch, click the compass at **(32,202)** for
Discover, confirm the capture shows the search field, then `windowfocus` the window, then type,
then capture the **whole frame** and crop only after seeing it. Three of this session's wrong
conclusions came from a crop taken before the frame was checked; the fourth came from this one.

### The rail's click targets, mapped after three aim errors (2026-10-31)

Three runs this session clicked the rail at a guessed y and hit the wrong page. The map, read off
the fifty-pixel pitch the rail actually uses:

| y | destination |
| --- | --- |
| 62 | Play / Home |
| **102** | **Discover** |
| 142 | Skins |
| 182 | Screenshots |
| 222 | Servers |
| 815 | Settings (gear, at the floor) |

So (32,120) landed on Discover -- which is why that run's capture showed the search field, and
its empty-after-typing result still stands as the evidence for the app-wide keyboard row. And
(32,202) landed on **Screenshots**, which is what this round's frame shows; that run proves
nothing and is discounted.

Every future run should click one of these numbers and then look at the frame before cropping,
which is now written down twice in this document because it has cost four rounds.

### Confirmed on a correctly aimed run, and one API dead end (2026-10-31)

Aimed at Discover (y 102) with the window focused by `xdotool windowfocus`, clicked into the
search field and typed `sodium`: the field still reads *Search modpacks...*. So the app-wide
keyboard row stands on a run whose framing was verified first.

The obvious eframe-side fix does not exist in this version: `ViewportBuilder` here has no
`with_focused` and no `with_active`, so the change was reverted the moment the compiler said so --
leaving the tree building is worth more than an experiment that cannot compile.

What remains to try, in order: a window manager in the container (openbox on :99) to prove the
app is fine and the lab is not; failing that, reading how this eframe/winit version decides a
window is focused before it routes key events, since the reference beside it receives them.

### A window manager changes the picture and does not fix it (2026-10-31)

`openbox` installed in the lab container and started on :99. Under it the launcher maps with a
titlebar and decorations, the content shifts down twenty points -- and **the app responds to
nothing**: a click on the rail and a typed `sodium` both leave the frame byte-identical (the two
captures hash the same). Before the window manager the app at least changed pages on a click.

So the keyboard failure and the new click failure share a cause, and it is on the app's side of
the window: this build is not processing input events in either configuration. That is a smaller
question than "does X deliver keys" and it is the first one worth answering -- whether the event
loop is driven by a repaint-on-demand path that never runs when the window is not considered
focused.

**Lab note**: the container now has `openbox`, which the committed Dockerfile does not. Either the
Dockerfile gains it or the lab's image is rebuilt before anyone trusts a capture taken in it --
recorded here because a capture lab that drifts from its recipe is how three of this session's
wrong conclusions started.

### The app is alive, repainting, and receiving nothing (2026-10-31)

Under openbox the frame did not change after a double rail click and a typed word -- and the two
captures hash identically, which is the important detail: `ps` shows the process alive at 9.9%
CPU for the full minute, so it is repainting at speed while **not one input event reaches it**.

The two configurations now read like this:

| lab | mouse | keyboard |
| --- | --- | --- |
| Xvfb only, no window manager | page changes work | nothing, ever |
| Xvfb + openbox | nothing | nothing |

So there is an input path that works (mouse, no WM) and fails as soon as a real window manager
owns the keyboard focus -- and a keyboard path that never worked in either.

**The next probe is one build and one line**: in `Shell::update`, print
`ctx.input(|i| (i.events.len(), i.pointer.any_click(), i.focused))` whenever any of them is
non-default. That answers whether the events arrive at egui at all, or die between X and the
toolkit -- and it is the same probe discipline that closed the window-defaults seam in one build
after reasoning had failed twice.

### The events arrive; the interface ignores them (2026-10-31)

The probe, run under openbox:

    input events=0 click=false focused=false   <- at start, window not focused
    input events=8 click=true  focused=true    <- after the click: focus taken, click delivered
    input events=2 click=false focused=true    <- typed characters arriving as Text events
    input events=1 click=false focused=true

**So egui receives the click, receives the characters, and knows it is focused.** And the frame
afterwards is byte-identical to the one before: same page, same breadcrumb, same everything.

That moves the fault out of X, out of focus and out of winit -- all three of which delivered what
they should -- and into this launcher's own drawing code. Two candidates, both cheap to check in
the source rather than in a capture:

1. the widgets are drawn on a painter whose layer is not interactive (the shell paints into a
   layer with `Sense::hover` or none at all), so a click never becomes a widget click;
2. the click position the shell tests is not the window position egui reports -- a coordinate
   transform applied twice, or an origin offset the shell adds for the title bar.

Given the shell draws a custom 48-point title bar and a custom rail rather than using egui panels,
(2) is the more likely: an offset the shell adds for its own chrome would put every click twenty
points away from where the reader aimed, which matches the click landing on nothing at all.

**The events prove the pipe is open. The next round should read how the shell converts pointer
positions into its own rects -- not the input path, which is now known to work.**

### The shell uses \`ui.interact\`, and the app is given a bare \`ui\` (2026-10-31)

The located suspect, read from the source: every clickable element in the shell is
\`ui.interact(rect, id, Sense::click())\` -- the rail buttons, the utility pair, the kind tabs, the
search selects, the pager cells, the result cards. That call registers a widget only when the
\`Ui\` it is made on owns an interactive layer and its clip rectangle contains the rect.

This app's entry point is

    impl eframe::App for App {
        fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) { ... }
    }

-- a bare \`Ui\` handed in, not a panel the app creates. Painting through it works, which is why
every capture in this session looked right; \`interact\` through it may register nothing, which is
why the events arrive (proved last round) and no widget reacts.

The experiment -- wrapping the shell in \`CentralPanel::default().show(&ctx, ...)\` -- did not
compile in the ten minutes left (`show` wants `&Context`, and the closure needs `&mut self` while
\`self.log\` borrows it), so it was reverted rather than left broken. **That is the one change to
make next**: draw the shell inside a panel, then click the rail and watch the breadcrumb.

### The panel wrap needs a different entry point, not a different borrow (2026-10-31)

Second attempt at drawing the shell inside a panel: splitting `&mut self` into `&mut shell` and
`&mut log` fixed the borrow, and the compiler then refused `CentralPanel::show(&ctx, ...)` with
"types differ in mutability" -- in this egui build the panel's `show` wants a **mutable** context
reference, and `eframe::App::ui` hands out an immutable one.

That is not a borrow puzzle to solve; it is the entry point saying the panel does not belong in
`ui`. The shape the API is pointing at is the other one:

    impl eframe::App for App {
        fn update(&mut self, ctx: &mut egui::Context, frame: &mut eframe::Frame) {
            egui::CentralPanel::default().show(ctx, |ui| self.shell.ui(ui));
        }
    }

**This is the next change and it should be the whole round**: implement `update` instead of `ui`,
give it the mutable context the panel wants, and click the rail. If the breadcrumb moves, the
eight-round thread is closed by a function signature; if it does not, the panel was not the cause
and the `ui.interact` hypothesis dies with it.

The tree is green and the two failed attempts are recorded rather than hidden, because the next
person will otherwise try the same wrapper and hit the same wall.

### The panel wrap compiles and changes nothing (2026-10-31)

eframe 0.36's `App::ui` hands the app a bare `Ui` and its own documentation says to wrap it in a
`CentralPanel` "to remedy this". In this version `CentralPanel::show` takes a **`&mut Ui`**, not a
context -- which is exactly what the two earlier attempts got wrong, and why they failed with a
mutability error rather than a borrow error.

The wrap is in the tree now and it compiles and paints identically. **It does not fix the click**:
two clicks on the compass leave the breadcrumb reading "Home".

So the panel was not the cause either, and the remaining suspect is the shell's own hit-testing:
the shell draws its own title bar and offsets everything below it, and if the rectangles it feeds
`ui.interact` are in a space offset by that title bar, every click lands forty-eight points from
where the reader aimed -- which is indistinguishable from "clicks do nothing" at the coarse
granularity of a rail button.

**The next probe prints, for the compass button only**: `response.rect` and the pointer position,
on any frame where the pointer is inside the window. Two rectangles and a point, and the offset
either shows itself or rules itself out.

# RETRACTION, 2026-10-31: the interface was never inert

The two-rectangle probe answered it:

    rail1 rect=[[8,100]-[56,148]]   pointer=[31,124]

The rail's compass button occupies content y 100..148, centre 124. Moving the pointer to screen
y 146 put it at content y 124 -- inside the button -- and **the click navigated**: the breadcrumb
changed from "Home" to "Discover modpacks".

So the last ten rounds of this session were chasing an artefact of my own aiming. Under the window
manager the window's content starts twenty points below the screen origin (its title bar), and
every click I aimed from the screenshots landed twenty to thirty-four points above its target.
Before the window manager existed, clicks worked too -- pages changed in those captures -- and the
"nothing responds" finding only appeared when I added the WM and kept the same screen coordinates.

**What survives, and it is one thing only**: *typing* does not reach a field. The keyboard events
arrive at egui (`events=2`, `focused=true` in the probe two rounds ago), the click now provably
lands, and the search field still shows its placeholder afterwards.

**What is withdrawn**: "no text field in this launcher accepts typing" as a P0 above every other
row. It was filed on nine rounds of mis-aimed clicks, and the file-management and interaction rows
it was going to condemn are untouched by it.

The lesson is not "aim better". It is that a coordinate taken from a screenshot is a hypothesis,
and this session treated it as a fact eleven times.

# RETRACTION TWO, 2026-10-31: the keyboard works too

    search rect=[[88,126]-[1116,174]]   pointer=[599,198]   clicked=false

The Discover search field sits at content y **126..174**. Every click of mine aimed at the field
went to y 198 or lower -- twenty-four points below it, and twenty below the window origin besides.
Clicked at its measured centre and typed `sodium`:

**the field shows `sodium`, with a caret and its clear button.**

So there is no input bug in this launcher. Not clicks, not keys, not focus, not the panel, not
`ui.interact`. Every one of those eleven rounds of hypotheses was a coordinate read off a
screenshot and treated as a fact:

| round | claim | what was actually wrong |
| --- | --- | --- |
| 29-31 | the Java field takes no typing | click aimed 24 points low |
| 32-34 | no text field in the app takes typing | same |
| 35-37 | the interface ignores input events | same |
| 38-39 | `ui.interact` on a bare Ui registers nothing | same |
| 40 | the panel wrap did not fix it | same |

What is left of that thread is nothing at all, and the P0 rows it created are withdrawn.

**The panel wrap stays** -- eframe's own docs ask for it -- but it fixed nothing, and this commit
says so rather than letting it look like the cure.

The real lesson, after eleven rounds: a screenshot gives you a coordinate in *screen* space, a
window manager shifts that space, and a widget's rect is in *content* space. The only reliable
aim is a rect printed by the widget itself. Every successful diagnosis this session -- the window
defaults, the section gap, the footer baseline -- used a printed number; every failed one used a
coordinate estimated from an image.

## P0 found by driving: every search returns "No results" (2026-11-01)

The first interaction this session has actually *performed* rather than photographed, and it
finds a real bug in one step:

1. Discover → click the search field at its own rect (content y 126..174) and type `sodium`.
2. The field shows `sodium`, the caret blinks, the clear button appears.
3. Three seconds later the pane reads **"No results found for your query!"** with the pager gone.

The same page, with no query, had paginated **956 pages** of results a minute earlier, so the
network, the feed and the card grid all work. Only the query path fails.

Cleared so far: `sift-net`'s `search_url_parts_with_facets` builds
`{base}/search?query=…&limit=…&facets=[["project_type:mod"]]&index=…` correctly, percent-encoding
the query and the facet groups. So the URL is right, and the failure is either in what the shell
passes down (an empty or stale query, a facet that excludes everything) or in how the answer is
read back.

**The next step is one probe in the search request path**: print the final URL and the parsed
result count. That distinguishes "the wrong query is sent" from "the right query is sent and the
answer is dropped", and it is the same shape of probe that has closed every other question this
session.

### What this costs the record

Twelve rounds went into an input bug that did not exist. This round took **one** interaction to
find a real one. The lesson is not subtle: the brief asks for behaviour verified by driving, and
every round spent photographing produced either a correct pixel count or a false alarm.

### Driving further: the clear button is inert, and the query test is still owed (2026-11-01)

Three interactions run against the live app, each aimed from a rect rather than a screenshot:

* **Clicking a pill switches tabs** -- "Mods" went from grey to brand-filled and the feed refetched.
* **Typing appends to the field** -- and `Ctrl+A` does **not** select, because the custom field
  implements no select-all; the query grew to `sodiumsodiumsodium` across two attempts.
* **The field's own clear button does nothing.** Clicked at its drawn position (screen 1090,170,
  which is the ✕ inside the field's right edge), the text stayed.

So the clear button is a real defect, distinct from the retracted input findings, and it is the
first one found by using the interface rather than photographing it.

**Consequence for the search bug**: it is *not yet* confirmed. Every run so far typed into a field
that already held text, so the API was asked for `sodiumsodiumsodium` and answered honestly with
nothing. A clean query needs the field emptied, and the field cannot be emptied by its button --
so the next run clears it with backspaces, which the widget does implement.

That ordering matters for the record: the "No results" sighting stays open *until a clean query
returns*, and the clear button becomes its own row with a reproduction.

# RETRACTION THREE, 2026-11-01: search works, and here is the proof it was never broken

Field emptied with backspaces (its own clear button being the one thing that does not work),
`sodium` typed once, Mods tab:

| result | author | downloads | hearts |
| --- | --- | --- | --- |
| Sodium | jellysquid3 | 258.13M | 42,362 |
| Sodium Extra | FlashyReese | 108.77M | 13,832 |
| Reese's Sodium Options | FlashyReese | 90.22M | 9,033 |

with tags (Optimization, Fabric, NeoForge, Quilt), an Install button on each card, and a pager
reading 1, 2, … 6.

**That is the first end-to-end interaction this project has verified with real data**: a query
typed into the field, sent to Modrinth, parsed, faceted by tab, rendered as cards. Every previous
claim about the Discover page was a claim about its pixels.

Withdrawn: "every search returns no results". It returned none because it was asked for
`sodiumsodiumsodium`, which is honest of it.

**What survives from this whole thread is one defect**: the search field's clear button does
nothing when clicked. One real bug in thirteen rounds of looking, and it was found by typing.

### The clear button is always visible now, and still does not clear (2026-11-01)

The reference shows the ✕ whenever the field has text; this drew it only while the pointer was
over the field, so a click arriving with the pointer freshly moved found nothing to hit. That is
fixed and the captures show the ✕ at the field's right edge without hover, on both sides of the
comparison.

**The click still does not clear.** Aimed at the ✕'s own centre (content 1098, screen 170, inside
its 16-point hit rect), the text stays `sodium` in the after-capture.

So the button exists, is drawn in the right place, and swallows its click somewhere between
`ui.interact` and `self.text.clear()`. The next probe is the same one that settled every other
question: print `hit.hovered()`, `hit.clicked()` and the rect for `Id::new("search-clear")` on
any frame with a click pending.

### The clear button fires; the text comes back (2026-11-01)

    clear rect=[[1090,142]-[1106,158]] pointer=[1097,148] hovered=true clicked=true

The click **is** delivered and `self.text.clear()` runs. The field then shows the old text again,
which means the text is put back between the widget and its owner -- a controlled-input seam, not
a hit-testing one. `Search` owns `text`, and the shell owns the query it passes down; if the shell
hands `sodium` back into the widget every frame, clearing the widget lasts exactly one frame.

So the row is now precise and belongs to the shell, not the widget:

**The clear button works; the field's text is re-seeded from the owner every frame.**

The fix is in the shell: either stop writing the query into the widget after the first frame, or
let the widget be the single owner and read `query()` back after `ui()` returns -- which the shell
already does at line 1469 for the request, so the direction of truth needs deciding once rather
than both being written.

# RETRACTION FOUR, 2026-11-01: the clear button works

    clear rect=[[1090,142]-[1106,158]]

clicked at its centre (1098) and the field empties to its placeholder.

The earlier attempts clicked **1090** -- the rectangle's own left edge -- and missed by a
half-pixel of rounding. So the last confirmed defect in the interaction thread was an aim error
too, and the count for that thread is now: **four retractions, no real defects**.

What that thread did produce, and it stands: the ✕ was visible only while the pointer was over the
field, where the reference shows it whenever there is text. That is a genuine fidelity fix, made
because the run before it forced a look at the condition.

The lesson, and it is the fourth form of the same one:

* a coordinate from a screenshot is screen space, not content space (retractions 1 and 2);
* a query typed into a field that already holds text is not a query (retraction 3);
* **an edge is not a centre** (retraction 4).

All four cost rounds. All four were avoidable with the probe that prints the widget's own rect --
which is now the first thing to do rather than the last.

## P0 found by driving: Install on a mod runs the modpack installer (2026-11-01)

Clicked **Install** on the Sodium card in a live search. The button works -- it fetches -- and then
reports, verbatim:

> Could not install Sodium: that archive is not a readable pack: invalid pack: neither
> modrinth.index.json nor manifest.json is present.

That message is the whole bug in one line. A **mod** is a `.jar`; the installer is looking for a
**modpack**'s `modrinth.index.json` or a CurseForge `manifest.json`. So the download path is
wired, the archive is fetched, and the wrong reader is applied to it.

The reference's behaviour, for comparison: Install on a mod opens a version picker (game version,
loader), asks which instance to install into, then writes the jar into that instance's
`mods/` folder and shows progress. None of those three steps exists here.

**Scope**: this is the brief's "file-management behaviour" row, and it is the first genuine
functional defect found in this session -- found by clicking, not by looking.

What it needs, in order: a version list per project (`/project/{id}/version`), a picker filtered by
the instance's game version and loader, and an install step that writes to
`{instance}/mods/`. All three exist as endpoints in `sift-net`; none is wired to this button.

### The mod-install path, scoped against the code (2026-11-01)

The defect has a shape now, and the primitives already exist:

* `sift-game/src/install.rs:1164` -- `pub fn install_file(wire, dir, candidate)`: the one that
  drops a jar into a `mods/` directory. Already tested (its own tests at `:1764-1800` install the
  same file twice and check the second does not duplicate).
* `sift-game/src/install.rs:1562` -- `pub fn install_pack_archive(...)`: what the Install button
  calls today, and where the "not a readable pack" error at `:1495`/`:1583` comes from.
* `sift-ui/src/install.rs` -- the job that drives it, and `failure_line` at `:315` which printed
  the sentence in the capture.

So the fix is a **branch, not a new subsystem**: a project whose file is a jar is a mod, and its
archive goes through `install_file` into the chosen instance's `mods/`; a project whose archive
carries `modrinth.index.json` or `manifest.json` stays on `install_pack_archive`.

What is genuinely missing is the **target**: a mod needs an instance chosen before it can land, and
the reference asks with a picker. Until that picker exists, the honest behaviour is: exactly one
instance -> install into it and say so; none or several -> say which is needed. That is a smaller
and more truthful slice than a silent failure, and it is testable without a picker.

**Next round's whole job**: that branch, with the two existing functions, a test that a jar lands
in `mods/` and a test that a pack still lands as a pack, then one driven capture of the same
Install click producing a success toast instead of the failure one.

### Why the Install button is not being "fixed" with a gate (2026-11-01)

Reading the call site settled a design question that a hurried patch would have got wrong. The
comment above the pack call already states the intent:

> A project that is not a pack fails here with the engine's own words ... which is the honest
> answer for the Mod and Resource Pack tabs: this button installs packs into instances of their own.

So the button is pack-only **by construction**, and the reference is not: on a mod it opens a
version picker, then asks which instance, then writes the jar. Two shortcuts were considered and
both are wrong:

* **Hiding Install on non-modpack tabs** would remove a control the reference shows -- a parity
  regression dressed as a fix.
* **Installing into "the one instance" when there is exactly one** is the smaller truthful step,
  but it is still an install without a version choice, and it silently picks a loader.

The real fix needs the picker, and the picker needs the version list (`/project/{id}/version`),
which `sift-net` already models. That is the next feature, not the next patch.

**What is wrong today and can be said in one sentence**: the failure message blames the archive
("not a readable pack") when the truth is that this button has no mod path at all. Fixing the
*sentence* is a five-line change in `failure_line` and is worth doing on its own; fixing the
*behaviour* is the picker.

### The message is fixed, and the capture proves it (2026-11-01)

Driven end to end: Discover, Mods tab, `sodium`, Install on the first card. The toast now reads

> **Could not install Sodium: this button installs modpacks, and installing a mod needs a version
> picker that is not built yet.**

which is true, names the reader's own project, and stops blaming the archive. Two unit tests hold
it: one asserts the pack-reader sentence is replaced, the other that every other engine reason
passes through whole -- because the engine's words are the only ones that say what actually went
wrong when something *is* broken.

The behaviour is still pack-only. What changed is that the launcher now says so.

### The version picker's first piece, tested (2026-11-01)

`pick_version(versions, game_version, loader)` is in `sift-ui/src/install.rs` with three tests:

* the newest version that fits the instance wins -- `1.20.4`/fabric takes 3.0, not the newest 4.0;
* nothing compatible still gets the newest build with a file, and the caller says which it took;
* a version with no downloadable file is skipped however well it fits.

The two signatures it sits between are already in the tree and were read rather than guessed:
`sift_net::modrinth::version_url(project)` at `modrinth.rs:250` and
`install_file(wire, target_dir, &version)` at `sift-game/src/install.rs:1164`, which takes a
`ModrinthProjectVersion` and drops its primary file into a directory.

What remains for the feature is the two ends: fetch the version list (the same download-and-parse
shape `search.rs` already uses), choose the instance (one -> it; none or several -> say which is
needed), and call `install_file` into `{instance}/mods/`. The middle -- which version -- is done
and tested.

### The picker's other end: which instance (2026-11-01)

`choose_instance(ids)` answers the second question a mod install has to ask, and both answers are
tested:

* **one instance** -> it, named back;
* **none** -> "there is no instance to install into. Create one first.";
* **several** -> "there are N instances and choosing one needs a picker that is not built yet."

The count is in the message because "no instances" and "three instances" are different problems,
and the reference's answer to the second is a modal that does not exist here. This is the honest
interim the matrix promised, and it is the same shape as `pick_version`: a pure function with the
decision in it, so the I/O that surrounds it is the only thing left to write.

Left for the feature: fetch `version_url(project)` with the same download-and-parse shape
`search.rs` uses, take `choose_instance` and `pick_version`, call
`install_file(wire, {instance}/mods, &version)`, and report the summary. Roughly thirty lines in
the job, and then the same driven Install click has to end in a success toast rather than a
message about a picker.

### The mod path runs end to end (2026-11-01)

`install_mod` is wired as the fallback for a project whose archive is not a pack, and the driven
Install click proves the path executes:

> **Could not install Sodium: there is no instance to install into. Create one first.**

That sentence can only come from the new code: the pack attempt failed with "not a readable pack",
the fallback ran, it enumerated the instances (there are none in this profile), and
`choose_instance` produced its tested refusal. So the chain works -- fallback, enumerate, decide,
report -- and the only thing standing between this and a real install is an instance to put the
jar in.

`install_mod` does the rest of the reference's job already: it resolves `version_url(project)`
through the same cached `wire.document` a pack install uses, parses the version list, hands it to
`pick_version` (empty game version and loader until an instance is chosen to read them from), and
calls `install_file` into `{instances}/{id}/mods`.

**Next step, and it is small**: create an instance through the UI, press Install again, and the
toast should read "Installed Sodium 0.x into …". That single run would make this the first feature
of the session verified by doing rather than by looking.

# DONE, by doing: a mod installs into an instance (2026-11-01)

The first feature of this project verified end to end by driving it, and the file on disk is the
proof:

1. **Create an instance** through the UI -- Home, "Create an instance" -> the Library shows
   "Instance 1, Vanilla 1.21.4" and a toast says "Created Instance 1."
2. **Search** Discover -> Mods -> `sodium` -> Sodium, Sodium Extra, Reese's Sodium Options.
3. **Press Install** on the first card. Eight seconds later:

> **Installed Sodium Sodium 0.9.3-beta.1 for Fabric 26.3 into Instance 1.**

4. **On disk**:

    /root/.local/share/SiftLauncher/instances/Instance 1/mods/sodium-fabric-0.9.3-beta.1+mc26.3.jar

That is the brief's "file-management behaviour" for mods, working: the version was resolved for
the instance, the primary file was downloaded, and it landed in the instance's `mods/` folder.

Two things it is honest to note. The toast says "Sodium Sodium" because the version's own name is
"Sodium 0.9.3-beta.1" and the project's is "Sodium" -- the reference prints the version name alone.
And `pick_version` was called with an empty game version and loader, so it took the newest build
rather than one chosen for the instance; the reference's picker is what replaces that, and the
function already accepts both.

**This is the row that changes the project's footing**: everything before it was pixels, and this
is behaviour with a file behind it.

### The install sentence, corrected and re-driven (2026-11-01)

Before: "Installed **Sodium Sodium** 0.9.3-beta.1 for Fabric 26.3 into Instance 1."

After, from the same driven click:

> **Installed Sodium 0.9.3-beta.1 for Fabric 26.3 into Instance 1.**

A version's own name usually carries the project's, so the rule is: print the version name alone
when it already begins with the project's, and prefix the project's name only when it does not --
which is what the reference does, and what two tests now hold (one for each case).

That closes the second of the three honest notes from the install milestone. The third -- that
`pick_version` was given an empty game version and loader -- still stands, and it needs the
picker rather than a sentence.

### The version is chosen for the instance now, and its proof is one jar name away (2026-11-01)

`install_mod` reads the chosen instance's own `version` and `loader` (`data::Entry`, fields in
`sift-ui/src/data.rs`) and hands them to `pick_version` instead of empty strings. The loader goes
in lower case because the API spells it "fabric"; "Vanilla" matches nothing and falls back to the
newest build, which `pick_version` documents and tests.

**The driven confirmation is still owed**, and the shape of it is unambiguous: the jar already in
the instance is

    sodium-fabric-0.9.3-beta.1+mc26.3.jar

which was chosen when the picker had nothing to go on. With the instance on Vanilla 1.21.4 the
same click must produce a **1.21.4** build instead -- a different filename in the same folder --
or, if Sodium publishes nothing for 1.21.4, the newest build again with the same warning the
fallback test describes.

The attempt this round missed the second card's button (no toast, no second jar), so the claim
rests on the tests and the code, not on a capture. That is written here rather than implied,
because this session's whole history says an unverified claim becomes a false one.

### A Fabric mod is refused for a Vanilla instance, proven both ways (2026-11-01)

The owed verification, run with the jar deleted first so the outcome could not be the old file:

* toast: **"Could not install Sodium: Sodium publishes no build for vanilla 1.21.4."**
* `instances/Instance 1/mods/` -- **0 files**.

So the instance-aware choice works in the negative direction, which is the direction that matters:
before this round the same click wrote `sodium-fabric-0.9.3-beta.1+mc26.3.jar` into a Vanilla
instance, where the game would have refused to start. Now it writes nothing and says why.

The reason the positive case did not show a 1.21.4 jar is the same fact seen from the other side:
a Vanilla instance has no Fabric build to match, so the honest outcome *is* the refusal. Proving
the positive needs a Fabric instance -- create one, then the same click should produce a
`+mc1.21.4` jar.

One nit, recorded rather than fixed in the same breath: the sentence prints the loader
lower-cased ("vanilla") because that is the form the API matching uses; the reference capitalises
it.

That closes the install milestone's third note, and with it all three.

### The positive install proof needs a Fabric instance, and the recipe to get one (2026-11-01)

Attempted this round and did not reach it: the click meant for Home landed on a page already
showing Discover, so no create flow opened.

What the next run needs, in order:

1. **Relaunch**, so the app opens on Home rather than wherever the last round left it -- every run
   in this session that assumed a starting page has wasted a capture.
2. Click **Create an instance** at (596,590). One press produced "Instance 1, Vanilla 1.21.4",
   which is the quick-create default; a **loader choice** must exist somewhere in that flow, and
   finding it is one capture of the modal rather than a guess.
3. With a Fabric instance present, the same Discover -> Mods -> `sodium` -> Install click should
   write a `+mc<version>` jar matching that instance, where the Vanilla case correctly refused.

Until then the install milestone stands as: **negative case proven** (refusal, no file), **positive
case proven once** (a 1.9 MB jar, before the loader rule existed), and the combination -- a jar
chosen *for* an instance and written -- still to be shown in one run.

### Home changes shape once an instance exists (2026-11-01)

Relaunched (the lesson from the last round) and the app opened correctly on Home -- but Home is
now the **Library**: a list with "Instance 1, Vanilla 1.21.4" and its play button, with no large
"Create an instance" call to action anywhere. The (596,590) coordinate that worked when the library
was empty is empty space now.

So the create affordance in this state is the **rail's "+"**, the fifth icon, at about
**(32,262)** -- the same button the reference puts under its four destination icons. That is the
next click to make, and the capture after it either shows a modal with a loader choice (the thing
the positive install proof needs) or shows that this launcher has no loader choice at all, which is
a finding in its own right and a parity gap.

Recorded instead of guessed at, because the last two rounds both lost their run to an assumption
about which page and which button were on screen.

### The rail map, corrected by a tooltip (2026-11-01)

Clicking (32,262) opened **Screenshots** -- the tooltip in the capture says so. The earlier map in
this document put Screenshots at 182, which was wrong, so the rail's pitch is not the 50 I assumed
for the lower icons.

What the frame shows, in screen coordinates (add 20 for the window manager's title bar):

| y | icon |
| --- | --- |
| 92 | Play |
| 142 | Discover |
| 192 | Skins |
| 262 | **Screenshots** (tooltip confirmed) |
| 316 | Servers |
| 394 | **+** (the create affordance) |

**Lesson**: a tooltip is a free label. Hovering before clicking costs one second and would have
saved three rounds of wrong rail coordinates -- and unlike a screenshot estimate it is the
application telling you what the control is.

## P1: creating an instance has no modal, so no Fabric instance can exist (2026-11-01)

Hovered (tooltip confirmed the button) and clicked the rail's **+**: the launcher created
**"Instance 2, Vanilla 1.21.4"** instantly, with the toast "Created Instance 2" and no dialog at
all.

That is a real parity gap with consequences beyond itself. The reference's create flow is a modal
with a **name**, a **loader** (Fabric/Forge/NeoForge/Quilt/Vanilla), a **game version** and the
option to start from a modpack. This launcher's is a single press that always produces the same
Vanilla instance, which means:

* **no Fabric instance can be created**, so the positive install proof (a jar chosen *for* an
  instance) cannot be demonstrated through the UI at all -- the refusal proven last round is the
  only outcome reachable;
* the mod picker's loader question has nothing to answer about, because every instance answers
  "Vanilla";
* and a reader who wants anything but Vanilla has no path in the product.

**So the next feature is the create modal, not the install picker** -- the picker has no meaning
until instances can differ. That reorders the remaining work, and the matrix now says so.

Side effect worth noting for the next run: there are now **two** instances, so `choose_instance`
will refuse with "there are 2 instances and choosing one needs a picker that is not built yet" --
the multi-instance branch that was only unit-tested until now.

### The create modal needs its capture before its code (2026-11-01)

Tried to read the reference's create-instance component out of the source tree and could not
reach it in this round: the checkout paths used earlier in the session (`.scratch/modrinth-0.22`,
`/scratch/...` inside the container) do not resolve from here any more. That is a small thing to
fix and it is the first step next round.

What is not in doubt is what the modal contains, because the reference's own flow is visible in
the product: **name**, **loader** (Fabric / Forge / NeoForge / Quilt / Vanilla), **game version**,
and the option to start from a modpack instead of an empty instance. This launcher has none of it.

The order that avoids another round of guessing:

1. **Capture the reference's modal** -- launch the reference, press its create button, capture. One
   run, and it fixes the layout, the field order and the default selections.
2. Build the modal to that capture, with a test per field and a test that the created instance's
   folder carries the chosen loader and version.
3. Drive it: create a Fabric instance, then repeat the install click, and the positive proof that
   has been owed since round 59 becomes reachable.

Recorded rather than started, because starting a modal from memory is how the last twenty rounds
of pixel work went.

### The reference did not answer two clicks either (2026-11-01)

Launched the reference, clicked its rail's plus at (32,394), then its "Create an instance" at
(596,590). Both captures are byte-identical to the page before the click -- the same hash -- so
neither press registered.

The reference is a different toolkit and the same rule applies to it that took twelve rounds to
learn about ours: **the first click on an unfocused window only focuses it**, and the app's own
window manager here is the one added this session. Two presses at the same point, or one
`xdotool windowfocus` first, is the shape of the retry.

So the create modal's capture is still owed, and the method is now written down rather than
implied:

    lab.sh ref
    xdotool windowfocus $(xdotool search --name "modrinth" | head -1)
    xdotool mousemove 596 590; click 1        # Create an instance on the empty Home
    shoot ref-create

That run is the next round's first act, and everything else about the modal waits on it.

### The reference click missed by fifty pixels, and the arithmetic (2026-11-01)

Third attempt at the create modal, still byte-identical. The cause is not the reference: the
preview I have been reading is **760 pixels wide for a 1440-pixel capture**, so one preview pixel
is 1.895 real ones, and the "Create an instance" button that sits at (298,285) in the preview is at
**(565,540)** on screen -- not the (596,590) I clicked, which is fifty pixels below and right of it.

The correct targets, converted once and written down:

| control | preview | screen |
| --- | --- | --- |
| Create an instance | (298,285) | **(565,540)** |
| rail plus | (16,187) | **(30,354)** |
| rail Home | (16,31) | (30,59) |

Every capture this session has been read through a preview that is not 1:1, and only some of my
coordinates were converted. That is the same class of error as the title-bar offset and the edge
clicked instead of the centre: **a number taken from a resized image is not a coordinate.**

# The reference's create modal, captured at last (2026-11-01)

`.scratch/cap/ref-create4.png` -- the gating capture owed since round 63, taken with the
converted coordinate (565,540) after a `windowfocus`.

It is a centred modal, 576x350 in screen points, titled **"Create instance"** with a close ✕ at its
top right, and it asks one question three ways:

**"Already know what you want to play?"** -- a search field, "Search mods, modpacks, and more…",
which is a project search that skips the rest of the dialog.

then "**or**", centred, and

**"Choose instance type"** -- four rows, each with an icon, a title and one line of description:

| row | title | description |
| --- | --- | --- |
| 1 | **Custom setup** | Start from scratch by picking a loader and game version |
| 2 | **Start from a mod or modpack** | Choose a project and we'll use its latest version |
| 3 | **Upload a modpack** | Install a modpack from an .mrpack file on your device |
| 4 | **Import instance** | Import an instance from Prism, CurseForge, or similar. |

Row 2 carries a chevron, because it opens a further step.

That is the whole create flow's shape: **the loader and game version live one level deeper**, behind
"Custom setup". This launcher's one-press Vanilla instance replaced all of it.

**Next round builds this modal to this capture** -- the four rows, the search field, the title's ✕ --
and then "Custom setup" is what has to exist for a Fabric instance, which is what the install proof
has been waiting on since round 59.

### The create dialog's four rows are in the code, transcribed and tested (2026-11-01)

`shell::INSTANCE_TYPES` holds the capture's four rows -- title, description, and whether the row
opens a further step -- with a test asserting all four, their order, and that only the first two
carry a chevron.

Two things the transcription taught, both now in comments:

* the reference's descriptions are **not uniformly punctuated** -- "Import instance" ends a
  sentence and the three above it do not -- so the test asserts presence, not full stops. My first
  attempt asserted every line ended with one and the test failed, correctly;
* the third row says **`.mrpack`**, and the first transcription typed `.mrpak`. A test cannot
  catch that; only a second read of the capture can, and did.

What is still missing is the drawing: the modal, its search field, its "or", and the state that
opens it instead of creating an instance on the press.

### The dialog's state exists; the drawing does not (2026-11-01)

`Shell` now carries `create_open: bool`, defaulting to false, documented as the flag the rail's
plus will set where it used to create a Vanilla instance on the press. Nothing reads it yet, and
the plus still creates instantly.

Three builds were spent on a misplaced brace: editing a struct's opening line with a string
replace left a second `{` and the compiler reported "unclosed delimiter" at the end of the file
rather than at the damage. The fix was a two-character edit, but the sequence is the session in
miniature -- a change made by pattern rather than by reading the surrounding lines, diagnosed by
probe afterwards instead of before.

**Next round draws the dialog** -- 576x350 plate, title and ✕, the search field, "or", and the four
rows already transcribed in `INSTANCE_TYPES` -- and flips this flag instead of creating.

### The create dialog is drawn in code; the capture has not caught it yet (2026-11-01)

`Shell::draw_create(ctx, screen)` paints the reference's dialog: a scrim, a 576x350 plate with a
one-pixel `--surface-5` edge, "Create instance" and a close mark, the rule under them, "Already
know what you want to play?" with its search plate and placeholder, an "or", "Choose instance
type", and the four rows from `INSTANCE_TYPES` at a 42-point pitch. It is called from the frame
after `shell.ui`, and the rail's plus now sets `create_open` where it used to create an instance
-- with `request_repaint()`, because this window only redraws while something is in flight.

**Two captures came back byte-identical to the page beneath**, so the dialog has not been seen on
screen yet. The code compiles, the tests pass, and the open path is one line; what is unproven is
which of the two ends is failing -- the click landing on the plus, or the foreground layer drawing.

The probe that settles it is the one this session keeps returning to: print `create_open` and the
plate's rect from inside `draw_create` on any frame, then click. Two numbers.

Four API guesses went into this round before the answer was to pass the screen rect in from the
frame (`ui.max_rect()`): this egui version keeps no `screen_rect` on the context, on the input, or
on `RawInput`. Reading the caller's own rect was the obvious move and the last one tried.

### The dialog never runs, so the click never lands (2026-11-01)

The probe printed nothing in a full run: `draw_create` is never called, so `create_open` is never
true, so the press at (32,394) is not reaching the rail's plus. The click, not the drawing, is the
broken end.

And (32,394) was itself derived from a **scaled screenshot** -- the same error this document has
now recorded five times. The rail's real button rects have only ever been printed once, for the
compass, and the rest have been estimated.

**So the next step is one probe, not one guess**: print the `create` rect and the pointer from the
rail's own code, exactly as was done for `rail1` two rounds before the compass started working.
Every control this session has found has been found that way; every control estimated from a
picture has cost a round.

# The create dialog is on screen (2026-11-01)

Captured at `.scratch/cap/r72-b.png`: a 576x350 plate over a scrim, "Create instance" and its
close mark, "Already know what you want to play?" with the search plate and its placeholder, the
centred "or", "Choose instance type", and the four rows -- Custom setup, Start from a mod or
modpack, Upload a modpack, Import instance -- each with its description.

Against `ref-create4.png` it is the same dialog in the same order with the same words. What
differs: the reference gives each row an icon and the second a chevron, and its plate is on the
reference's own raised surface; ours has neither icons nor chevron, and its row text sits two
points higher. Those are the next slice's work, not this one's.

**And the reason it took three rounds is worth the paragraph.** The `main.rs` edit that calls
`draw_create` never applied: the anchor string did not match, `String::replace` did nothing, and
nothing complained. The probe then correctly reported that `draw_create` was never called, and I
read that as "the click is broken" and spent a round printing rail rectangles. The click was
always fine; the call was missing.

This session has now recorded the same failure twice in different clothes: a silent `.replace()`
that matches nothing looks exactly like a change that did not work. Every edit of this kind needs
its own assertion -- the earlier commits in this file did assert, and the ones that skipped it are
the ones that cost rounds.

### The dialog closes, and the press that opens it no longer dismisses it (2026-11-01)

Two captures, the same pixels sampled in both:

| sample | open | after Escape |
| --- | --- | --- |
| (455,450) -- plate edge | #45464b | #16181c |
| (720,280) -- plate interior | #151618 | #27292e |

So Escape closes it, and the scrim and plate are gone rather than merely repainted.

The first attempt at this was wrong in a way only a capture could show: the dismissal ran before
the plate was built, so **the press that opened the dialog was also a click outside it** and closed
it in the same frame. The capture pair came back byte-identical and looked like "the dialog never
opens", which is what the previous round's file already suspected. The fix is a timestamp: the
dismissal is ignored for a tenth of a second after opening, so the opening press cannot be the
closing one.

Still missing, and next: the close mark and the four rows answer nothing. Escape and an outside
press are two of the four ways out; the other two are the ✕ and a chosen instance type.

### The rows answer, and the choice shows (2026-11-01)

Driven: open the dialog from the rail's plus, click the first row at its centre, capture. **Custom
setup carries a brand ring** and the three below it do not -- `.scratch/cap/r74-a.png`, cropped over
the rows.

So the dialog now has three of the four ways out the reference has: Escape, a press outside, and
the close mark, plus a row that remembers being chosen. What none of them does yet is *act*: the
ring records an intention, and every one of the four flows behind these rows -- custom setup's
loader picker, the project search, the .mrpack chooser, the Prism import -- is still to come.

The first sampling of this capture read the wrong column (`x 434`, two points left of the row's
own edge at 448) and reported "both edges the raised background", which would have been a false
negative. Looking at the crop settled it; the pixel probe, aimed properly, is what should have.

### Custom setup is a step you can walk to (2026-11-01)

Choosing the first row now moves the dialog to a second step, captured at
`.scratch/cap/r75-a.png`: the same plate and title, then **Name** `Name your instance`,
**Loader** `Fabric` and **Game version** `1.21.4`, each a label over a field.

So the flow navigates: plus opens the dialog, a row moves it on. What the step is not yet is
*editable* -- the three fields are painted, not placed, so nothing can be typed into them, and
there is no Create button to finish with.

That is deliberately the smaller half. The fields want `place_field` (which this launcher already
has, and which took twelve rounds to prove), the loader wants a select, the version wants a list
filtered by the loader, and Create wants the instance-writing path that already exists in
`data`. Each is a known quantity; none of them is guessed at.

**The next slice is those three fields taking input**, because a picker whose fields cannot be
typed into is the exact half-truth this session spent rounds proving was not true of this codebase.

### The name row has a real field; its typing is unproven (2026-11-01)

`create_name: String` is on the shell, and the name row now hosts an `egui::TextEdit` placed
through a foreground `Area` over the painted plate rather than a painted placeholder.

Driven once: open, choose Custom setup, click the field, type "Fabric Test". **The capture still
shows the placeholder**, so the field is not taking input -- and this round has no budget left to
find out why.

The next probe is the one that has settled every other question of this shape: print the Area's
rect, `response.hovered()` and `response.clicked()` from inside the `ui.put`, then click. Three
values, one build. Two candidates, both seen before in this session: an Area whose rect does not
line up with the painted plate (so the click lands beside the widget), or a widget that never
takes focus because nothing requests it.

Recorded as unproven rather than claimed, because a field that draws and a field that types are
the same picture -- the distinction that cost this session twelve rounds and its two retractions.

### The name field is drawn at (0,0), off its plate (2026-11-01)

The three-value probe, first try:

    name rect=[[0,0]-[544,28]]  hovered=false  clicked=false  focus=false

So the `TextEdit` exists and sits at the **screen's top-left corner**, not on the plate at
y 357: the `Area`'s `fixed_pos` positions the area, but `ui.put`'s rect is interpreted inside it,
and the response's rect comes back in the area's own space. Nothing was ever under the click.

The fix is not another `Area` argument. The frame already has a `Ui` at real coordinates; the
dialog should take `&mut Ui` and place its fields with `ui.put(absolute_rect, …)` the way
`place_field` does in the settings modal -- the path this launcher has already proved works, and
the one that took twelve rounds to find so that this round would not have to.

`draw_create(ctx, screen)` becomes `draw_create(ctx, screen, ui)`, called before the panel
closure takes `ui` by `&mut`, and the field placed on it directly. One signature, one call site,
one field.

### Placed on the frame's ui, still no typing (2026-11-01)

The Area was removed and the field placed with `ui.put(name_field, ...)` on the Ui the frame draws
through. Driven again -- open, Custom setup, click the field, type "Fabric Test" -- and the capture
**still shows the placeholder**.

So the coordinate space was not the only fault, or not the fault at all. What is known: the field
compiles, is placed at an absolute rect on a real ui, and receives nothing.

**The next probe is the same three values, re-taken on this placement**: `response.rect`,
`hovered()`, `clicked()` from inside the `ui.put`. If `rect` is now the field's own position and
the click still does not hover it, the ui it is placed on is not the one taking input -- the
settings modal's fields work through the same `place_field`, so the difference between the two call
sites is the thing to compare, and that comparison is cheaper than any further guess.

Recorded plainly: two rounds on this field, both ending without typing. The one thing that has
worked every time is the probe; the thing that has failed every time is changing the code first.

### Three of the four conditions hold: right rect, hovered, focused (2026-11-01)

    name rect=[[448,357]-[992,385]]  wanted=[[448,357]-[992,385]]
         hovered=true  clicked=false  focus=true

So on the frame's ui the widget sits exactly where it is drawn, the pointer hovers it, and **it holds
the keyboard focus**. Only `clicked` reads false, and that is expected: the probe prints every
frame and the click landed on an earlier one.

What is left, then, is the shortest link in the chain: the characters arriving while focused are not
reaching the painted text. The next probe is two values rather than three -- print `name` right
after the `ui.put` and `self.create_name` at the top of `draw_create` -- which says whether the
keystroke is applied to the local string and lost on the way back to the shell, or never applied at
all.

That comparison is the last cheap step before the field works, and the code around it is now
correct by measurement rather than by hope: rect, hover and focus are all confirmed on screen.

### The field's state works; its paint does not (2026-11-01)

The decisive probe:

    name local="abc"  stored="abc"

So keyboard input **reaches the widget and is stored on the shell** -- the field is functional in every
sense except that it draws nothing. Removing the duplicate placeholder made it plainly empty, and an
explicit `.text_color(theme.CONTRAST)` changed not one pixel: the two captures hash identically.

That leaves one line of enquiry for the next round, and it is narrow: a `TextEdit` placed on the
frame's `Ui` **after** `CentralPanel::show` has returned is drawn outside that panel's clip
rectangle, so egui culls its glyphs while still delivering input to it. The fix is to place the field
inside a `show` block -- an `Area` positioned with `fixed_pos(name_field.min)` and the widget put at
`Rect::from_min_size(Pos2::ZERO, size)`, which is what the earlier attempt did *before* the rect
was known to be wrong.

The distinction this round earned: **input works, painting is culled.** Those are separate faults
and the session has been treating them as one.

### The field is in its own area and still paints nothing (2026-11-01)

Moved into a foreground `Area` at the field's own minimum, with the widget put at the origin of its
space. The capture hashes identically to the previous three: no glyphs, and this run's typing did
not register either.

So both mount points fail to paint, and one of them is known to receive input. What the settings
modal does differently is not the placement -- it is that `place_field` is called from inside a
**panel's `show` block**, on the `ui` that panel builds, with the modal's own painter drawing the
plate beneath. This dialog paints through a raw `layer_painter` and then places widgets outside any
panel.

**The one comparison that has not been made, and the honest next step**: give the dialog a real
`Area` that contains the whole modal -- plate, text, fields -- so the fields share the area's ui and
layer instead of being bolted onto a painter's output. That is how the settings modal is built, and
it is the only known-working shape in this codebase.

**Recorded as the session's open sore**: eight rounds have now touched this field, and every one
ended by naming a different suspect. The facts that are solid are two -- input reaches the widget
and is stored; no mount has produced a glyph -- and both are in this document with their probes.

### Two more mount points, the same empty plate (2026-11-01)

The dialog is now called **inside the panel's `show` block**, on the panel's own `ui` -- the shape
the settings modal's working fields use -- and the app **forces a repaint every frame while it is
open**, so a keystroke cannot be lost to the redraw budget. Neither changed one pixel: the capture
hashes identically to the previous three.

That eliminates the two remaining explanations this session had for the empty field: it is not
outside the panel's clip, and it is not a stale frame. Input reaches the widget (proven: the state
held "abc"), the widget is in the right place (proven: rect 448,357-992,385), and it never paints.

**Hand this thread to a fresh session with these three facts and nothing else**, because eight
rounds of suspects from this one have each been disproved by the next:

1. input reaches the `TextEdit` and is stored (probe: `name local="abc" stored="abc"`);
2. its rect is exactly the painted field's (probe: rect equals wanted);
3. it paints nothing, from three different mount points, with an explicit ink colour.

The most likely remaining cause is something this session has not looked at: the `Frame`/`Area`
visuals the field inherits, or the font atlas not being installed on that layer. A fresh pair of
eyes with those three facts will be faster than a ninth variation.

### A diagnostic field, and a crop that missed it (2026-11-01)

Placed the same `TextEdit` at a fixed rect near the panel's top-left, so that "does this widget
paint at all" could be answered without any of the modal's geometry in the way. The capture was
taken and then **cropped from the wrong region** -- (0,430) where the field was at (20,40 screen) --
so the diagnostic answered nothing.

That is the fourth aim error of the session and the second in two rounds, and it is worth naming
plainly: the crop coordinates were guessed rather than derived from the rect that was already
printed two lines earlier.

The diagnostic is reverted and the tree is green. The three facts about the field stand unchanged,
and the next attempt should read the field's own rect before cropping -- or better, capture the
whole frame and look at it, which has never once failed to answer the question being asked.

# SOLVED after ten rounds: the plate was drawn over the field (2026-11-01)

The Name field types. `.scratch/cap/r84-c.png` shows **"Fabric Test"** in the field with its focus
ring, inside the create dialog.

**The cause was layer order.** The dialog's plate was painted with
`ctx.layer_painter(LayerId::new(Order::Foreground, …))` while the field was placed with
`ui.put` on the panel's `ui` -- the middle layer. The plate therefore drew *on top of* the widget:
input reached it, it had the right rect, it held focus, and it was painted underneath an opaque
rectangle. Every symptom this session chased follows from that one line.

**What found it, after nine rounds of not finding it**, was the diagnostic that put the *same
widget* at a rect **outside** the plate: it painted perfectly. Same widget, same ui, same call --
only the position differed, and the only thing special about the position was whether the plate
covered it.

The fix is to draw the plate with `ui.painter()` so plate and field share the panel's layer, with
the field put second.

**The lesson, and it is the session's most expensive one**: nine rounds reasoned about *how* the
widget was mounted -- panel, area, foreground, repaint budget, clip rects -- and none of them
considered *what was painted over it*. A control that does not appear is as likely to be covered as
misplaced, and the test for that is one widget at a coordinate nothing else occupies.

### The dialog submits: Create makes an instance (2026-11-01)

Driven end to end: rail plus -> Custom setup -> **Create** -> the Library gains **Instance 3** and the
toast reads "Created Instance 3."

So the create flow is a flow: it opens, it offers a type, it takes a step, and it finishes. What it
does **not** yet do is use what the step collected -- the instance is Vanilla 1.21.4 whatever the
loader row says, and the typed name is ignored -- because Create calls the path the rail's plus used
to run, which takes no arguments.

That is the next slice and it is small: pass `create_name` and the chosen loader/version into the
instance-writing path in `data`, and the three rows stop being painted constants. Until then the
dialog is honest about what it does (it creates an instance) and silent about what it ignores (the
fields), which is the better of the two failure modes but still a gap worth naming.

### The dialog's name is used, and the loader now lies (2026-11-01)

Typed "Fabric Test", pressed Create, and the Library's first row reads **Fabric Test / Vanilla
1.21.4**. The field means something.

The second half of that row is the problem: the dialog's Loader row says **Fabric**, and the instance
it produced is **Vanilla**. `Instance::create` takes a name and a Minecraft version, not a loader, so
the loader was never wired -- and until it is, the dialog makes a claim it does not keep.

Two honest options, and the smaller one is next:

1. **stop the Loader row claiming a value** until it is wired -- paint the reference's own default
   without a value, so nothing is asserted that is not true; or
2. wire it: `Instance::create` grows a loader, which is a change to `sift-core` and to the instance
   profile it writes.

Option 1 costs a line and removes a false statement from the product; option 2 is the feature. Both
are better than the current state, which looks finished and is not.

### The loader row tells the truth now (2026-11-01)

The custom-setup step reads **Name** · *Name your instance*, **Loader** · *Vanilla*, **Game version** ·
*1.21.4* -- and Vanilla/1.21.4 is exactly what pressing Create produces. The row that said "Fabric"
while making a Vanilla instance is gone.

That is the last false statement the dialog was making, and it cost one line. The feature behind it
is still owed: `Instance::create` takes a name and a Minecraft version, not a loader, so a Fabric
instance cannot be created until that signature grows. Until then the row names the truth rather
than a wish.

### The loader, scoped to the exact contract (2026-11-01)

Reading both ends settles what "wire the loader" means, and it is smaller than it looked:

* **writer**: `Instance::create` (sift-core/src/instance/mod.rs:104) writes
  `PackProfile::vanilla(mc_version)` to `mmc-pack.json`.
* **reader**: `data::loader_and_version` (sift-ui/src/data.rs:98) walks `profile.components()` and
  maps uids to names -- `net.fabricmc.fabric-loader` -> Fabric, `net.neoforged` -> NeoForge,
  `net.minecraftforge` -> Forge, `org.quiltmc.quilt-loader` -> Quilt, everything else Vanilla.

So a Fabric instance is a profile with one extra component: uid `net.fabricmc.fabric-loader` and a
loader version. Nothing else in the product needs to know.

**The change, in three steps**: give `PackProfile` a way to add a component (its `components()`
exists, so a `components_mut()` or a `with_component` constructor is the shape); add
`Instance::create_with_loader(dir, name, mc_version, uid, loader_version)` that calls it; and have
`create_instance` pass the dialog's loader. A test asserts the written `mmc-pack.json` carries the
uid, which is the only thing the reader looks for.

The loader *version* is the one fact still missing -- it has to come from a Fabric meta endpoint or a
pinned default -- and that is a decision, not a discovery.

### The loader is a control now, not a label (2026-11-01)

Three chips under the Loader row -- **Vanilla**, **Fabric**, **NeoForge** -- with the chosen one
brand-filled, and the Loader row reporting the choice. Captured; the default is Vanilla, which is
what the dialog still creates.

Two limits, both stated in the code and here: the click that changes the chip has not been
demonstrated in this round (the press aimed at Fabric landed between chips), and the chosen value is
not yet written into the instance profile -- it changes the dialog and not yet the product.

That second limit is the three-step change the last note scoped: a component in `mmc-pack.json`
with uid `net.fabricmc.fabric-loader`, which is the only thing `loader_and_version` reads.

# A Fabric instance exists on disk (2026-11-01)

The chip click, the Create press and the profile write all landed, and the proof is a file rather
than a screen:

    instances/Instance 5/mmc-pack.json -> net.minecraft, net.fabricmc.fabric-loader

Every other instance in the folder carries `net.minecraft` alone. So the dialog's loader choice now
reaches the profile -- the component uid `loader_and_version` reads back -- and it does so through the
three steps scoped two rounds ago: a chip control, a component written after creation, and a pinned
loader version (0.16.9, with a meta lookup as the follow-up).

Two things this does and does not settle. It settles that a Fabric instance can exist, which is what
the install proof has been waiting on since round 59. It does not settle the loader *version*: the
profile claims 0.16.9 because that is pinned, not because anything asked Fabric what the current
loader is.

**Next run**: create a Fabric instance, then install Sodium into it and watch the jar land in
`Instance N/mods/` -- the positive case that the Vanilla refusal has been standing in for since
round 59.

### The install proof: precondition met, run not landed (2026-11-01)

The lab was trimmed to the one Fabric instance this session created -- `instances/` now holds
`Instance 5` alone, so `choose_instance` has an unambiguous answer -- and the install was driven
twice with longer waits. **No jar, no toast.** The sequence did not reach the button; nothing about
the install path is contradicted, and nothing about it is confirmed either.

The recipe that worked when this proof last succeeded (round 56, into a Vanilla instance) was:
Discover at (32,102), the Mods pill at (215,120), the search field at (600,170), `sodium`, then
Install on the first card at (1062,284). Today's runs used exactly that and produced nothing, so
something earlier in the chain is now off -- most likely the page state after a fresh launch with a
single instance, which shows the Library rather than the empty Home the earlier run started from.

**Next run should confirm each step from the frame before taking the next**: launch, capture, check
the page, then click. Two rounds have now been lost to assuming which page the app was on, and the
matrix has said so twice.

# CLOSED: a mod installs into a Fabric instance, chosen for it (2026-11-01)

The proof owed since round 59, and it holds with every change made since:

    toast:  Installed Sodium 0.6.13 for Fabric 1.21.4 into Instance 5.
    disk:   instances/Instance 5/mods/sodium-fabric-0.6.13+mc1.21.4.jar

Two things this proves at once. The install path works end to end -- search, version choice,
download, write -- and **the version was chosen for the instance**: 0.6.13 for 1.21.4, not the
newest 0.9.3-beta.1 for 26.3 that the same click installed before the picker existed. The Fabric
component written by the create dialog is what made that choice possible.

The run before this one failed for a reason worth recording with it: the compass is at **y 142**,
not the 102 used in the two failed attempts. 102 is Play. The rail map in this document was right
and the run ignored it.

State of the lab after this round: one instance (`Instance 5`, Fabric 1.21.4) holding one mod. The
five earlier instances were removed to give `choose_instance` a single answer -- lab data, created
by this session, not the reader's.

### The card knows what is installed (2026-11-01)

After the install, the Sodium card's button reads **"+ Installed"** in the brand colour where it read
"Install" before. The loop closes in the interface as well as on disk: instance created, project
found, version chosen, file written, and the card reporting it.

That is the fourth thing this session has verified by driving rather than by looking -- after the
search results, the refusal for a Vanilla instance, and the jar landing in `mods/`.

### The loader version is a named default, not a fact (2026-11-01)

`FABRIC_LOADER_DEFAULT = "0.16.9"` now sits beside `INSTANCE_TYPES` with a doc comment that says
three things plainly: it is **a default rather than a value read from anywhere**; the endpoint that
would supply the real one is `https://meta.fabricmc.net/v2/versions/loader`, whose first entry is the
current stable loader; and wiring it is a network call in the create path, recorded here rather than
implied by a bare number in a JSON literal.

That is the whole of the change, and it is deliberately small. The behaviour is unchanged -- a
Fabric instance is still given 0.16.9 -- but the claim is now bounded: a reader of the code can see
that nothing asked Fabric, and a reader of this document can see the row.

**Open row**: fetch the loader version from that endpoint at creation, and fall back to the default
only when the fetch fails -- with the failure surfaced, not swallowed.

### The pinned loader was stale, and asking proved it (2026-11-01)

`curl https://meta.fabricmc.net/v2/versions/loader` from the lab answers, first entry:

    {"separator":".","build":1,"maven":"net.fabricmc:fabric-loader:0.20.1","version":"0.20.1","stable":true}

So the current stable Fabric loader is **0.20.1**, and the default this session has been writing into
every instance profile was **0.16.9** -- several releases stale, guessed rather than asked.

The constant is now 0.20.1 with the measured response in its comment, and the lookup is still the
open row: a default that happens to be current today is not the same thing as asking. What changed is
that the number is now a measurement with a date on it, which is the most this round's budget allows.

### The Minecraft default was stale too (2026-11-01)

The same test applied to the other remote fact: `https://piston-meta.mojang.com/mc/game/version_manifest_v2.json`
answers `"latest": {"release": "26.3"}`, and this launcher's `DEFAULT_MC_VERSION` was **1.21.4** --
which was not chosen as a default at all, but was the version the install proof happened to use.

Corrected to 26.3 with the manifest quoted in the constant's comment. The pattern is now explicit
and worth carrying: **two constants that mirror remote facts were both wrong, both found by asking,
and neither had anything in the codebase that could have caught it.**

**Owed**: an install against a 26.3 instance, because the proven install ran on 1.21.4 and Sodium
publishes `+mc26.3` builds too -- the earlier vanilla-instance run picked exactly one -- but that
path has not been re-walked since this default changed.

# The debt is paid: the install re-lands on the corrected defaults (2026-11-01)

    toast:  Installed Sodium 0.9.3-beta.1 for Fabric 26.3 into Instance 2.
    disk:   instances/Instance 2/mods/sodium-fabric-0.9.3-beta.1+mc26.3.jar

The version chosen is **26.3**, the measured Minecraft default, and the instance's profile carries
**0.20.1**, the measured Fabric loader. So both corrected constants are load-bearing and both work:
the create dialog writes them, `loader_and_version` reads them back, `pick_version` chooses against
them, and the jar that lands matches.

That makes four installs this session has proven with a file, on four different profiles -- Vanilla
1.21.4, Fabric 1.21.4, and now Fabric 26.3 twice -- and it closes the one debt the last round opened
by changing a default the earlier proof had used.

### The parity tooling passes the audit the lightness harness failed (2026-11-01)

After repairing `tools/lightness-measure.sh`, the same question put to the tools this document
depends on: do they measure what they claim?

`tools/parity/lab.sh` is clean, and one line is why: it starts the container with
`docker run … -v "$ROOT":/scratch`, so every path inside is the checkout that invoked it. There is no
absolute path from another machine anywhere in the file, and the reference is launched from a path
under `.scratch/` that the same script creates.

So the tooling situation is now: one harness repaired (with two precondition guards), one lab script
audited and clean, and one `probe.py` whose only external dependency is a capture file it is handed.

**What remains untrustworthy is not the tools but the coordinates fed to them** -- five aim errors and
four false P0s this session, every one from reading a position off a picture. The tools have always
been able to answer; they were being aimed by eye.

### The totals quoted in these reports are not re-derived (2026-11-01)

Every round report this session has quoted the matrix as "118 DONE / 472 OPEN". Those figures came
from a count taken early and were **never re-counted as rows were added, closed or retracted**. A
first attempt to re-count them this round parsed the wrong column -- it grouped by element name
("FILES 42", "SCREENSHOTS 40") instead of by status -- and rather than publish a second wrong number,
the attempt is recorded here as unfinished.

That is the session's own lesson turned on its ledger: **a number that is repeated is not a number
that is measured.** Until someone counts the status column properly, the totals in the round reports
should be read as approximate, and the rows themselves -- each with its own evidence -- are the
authoritative record.

### The totals, counted at last -- and the session overstated its own progress (2026-11-01)

Counting every table row that carries a status token:

| status | rows |
| --- | --- |
| DONE | **93** |
| OPEN | **512** |
| BLOCKED-SERVICE | 9 |
| RENDERER-RISK | 16 |
| NOT-IN-SOURCE | 13 |
| DELIBERATE | 2 |

**633 status-bearing rows** (the per-status counts sum higher because some rows carry two, such as
OPEN and RENDERER-RISK together).

**CORRECTED 2026-11-01: these numbers do not describe this file's rows.** A scripted count of the numbered rows (§Counts)
gives **625 rows** — 86 DONE and 504 OPEN at HEAD — and the table above is 7 DONE and 8 OPEN higher, with one extra of
each other status and 2 DELIBERATE that live in a ledger table rather than in a row. What happened is the same mistake in
a new coat: it counted every table in the document that mentions a status token instead of the rows that carry one, which
is why its own sum (93 + 512 + 9 + 16 + 13 + 2 = 645) is larger than the 633 it prints and why it needed a note about rows
"carrying two". **The direction of its conclusion survives — "118 DONE" overstated completions — but none of its numbers
do.**

So the figure this session repeated -- "118 DONE / 472 OPEN" -- **overstated completions by 32 rows and
understated the open work by 32**. The ratio at that point was **86 done against 504 open** -- read from §Counts, not from
the table above it -- so for every element verified, five and a half remained.

That is the session's most useful number, and it is the one it had been getting wrong in its own
favour while telling the reader it was measuring.

### The library shows what the dialog wrote (2026-11-01)

Aimed at the instance page and missed it -- the click landed on the card without opening it -- but the
frame answered a different question: the card reads **"Instance 2 / Fabric 26.3"**.

That is the create dialog's two writes arriving in the interface: the loader component the profile
carries and the Minecraft version the manifest gave. Before this session the same card read
"Vanilla 1.21.4" for every instance, because every instance was one.

**How an instance page opens is now an open question with a named shape**: the card's body did not
open it, so it is either the ⋮ menu beside the play button or the play button itself. One capture of
each settles it, and it must precede any work on the page -- the brief's file-management requirements
live behind that door.

## P0: an instance cannot be opened at all (2026-11-01)

Two attempts, both negative:

* a press on the **card body** does nothing -- no navigation, no selection;
* a press on the **⋮ beside the play button** opens no menu.

So this launcher has **no way into an instance**. The reference's card opens the instance page from its
body and its overflow menu carries the per-instance actions; here the body is inert and the menu is
painted, not placed.

That makes the Instance page not merely unbuilt but **unreachable**, and it is the reason the brief's
file-management clause has no entry point: `mods/`, `config/`, worlds and logs all live behind a door
with no handle. The jar this session installed into `Instance 2/mods/` is on disk and invisible from
the product.

**The next slice is the door, not the room**: make the card body open the instance page (the shell
already tracks `open_instance`), then the ⋮ menu as a popup with the reference's items. Only then does
building the page itself make sense.

### The door exists; nothing turns the handle (2026-11-01)

Read rather than guessed, and it changes the last round's finding from "unbuilt" to "unwired":

* **`Shell::open_instance_page(&id)`** exists (shell.rs:1373) and is what puts the shell into the
  instance page (`open_instance` at :481, drawn at :634).
* It is reached by **`Action::Edit(id)`** (:1325), whose comment already says it is the reference's
  instance page.
* The library's card has an overflow menu with **`MenuItem::OpenFolder`** and siblings (:279), so the
  menu's items are modelled too.

What is missing is one press: **the card's body never emits `Action::Edit`**, and the ⋮ never opens a
popup, so the menu's items cannot be reached. The plumbing is complete end to end and unconnected at
the only place a reader can touch it.

**The next slice is a single interaction**, not a page: emit `Action::Edit(id)` from a press on the
card body, then capture the instance page it opens. Everything the brief asks of file management sits
one click beyond that.

### The press is wired and the page still does not open (2026-11-01)

The card now registers its own press -- `ui.interact(card, ("card", id), Sense::click())` before the
play pill and the kebab, emitting `Action::Edit` -- and a press at the card's centre still leaves the
Library on screen. The frame is byte-identical to the one before the change.

Two candidates, and the probe that separates them is the same shape as every other that worked:

1. **the press is not landing on the card's rect** -- print `card` and `response.hovered()`, exactly as
   the rail and the name field were settled;
2. **the action is landing and not being acted on** -- print the action at the top of `Shell::act`,
   which would show `Edit("Instance 2")` arriving and going nowhere.

Given that `Action::Edit` already calls `open_instance_page` and that function already sets
`open_instance`, (2) would mean the match arm is not reached -- worth knowing, because that same
dispatch carries Play, Duplicate and OpenFolder.

**Recorded rather than claimed**: one press wired, no page opened, two named candidates, one probe.

# The instance page is reachable, and it was built all along (2026-11-01)

The card's press works once it is aimed at the card's own rect: **`[[88,120]-[1116,216]]`**, centre
content (602,168) -- my press at screen (400,130) was ten points above its top edge, the session's
sixth aim error and its third in the instance doorway.

What it opens is not an empty page:

* a header reading **Instance 2 / Fabric 26.3** with its install path and a **Play** button;
* tabs **Content, Files, Screenshots, Worlds, Logs**;
* a Description panel listing **Loader: Fabric, Game version: 26.3, Last played: --, Path: ...**.

So the Instance page was not "untouched": it exists, is populated from the profile the create dialog
wrote, and was simply **unreachable**. Three rounds of this session called it unbuilt; one press at a
printed coordinate shows it complete enough to have a Files tab -- which is where the brief's
file-management behaviour lives.

**Next**: press **Files** (and **Content**) and drive them, then compare each against the reference's
own instance page, which has never been captured either.

### The instance page's five tabs do not answer (2026-11-01)

Pressed **Files** at its drawn position: the frame is byte-identical to the one before. So the page
renders its header, its tabs and a Description panel, and **only the Description panel exists** -- the
tabs are painted, not placed, which is the same shape as the create dialog's field and the card's
overflow menu.

That matters for the brief directly: **Files is where file management lives**, and it is a label on
this page rather than a place. The jar installed into this instance's `mods/` folder is still only
visible on disk.

**Next**: the same probe as every other -- print the tab rects and `hovered()` -- then wire the press
to a tab index and draw the Files view. The page's own scaffolding (header, path, Play, tab bar) is
already there and already reads from the profile.

### Correction: the tabs are wired; the Files view is a stub (2026-11-01)

Read the source after the press failed, and it overturns the last note twice:

* **the tabs are interactive** -- `ui.interact` at `pages/instance.rs:294`, dispatching to a per-tab
  renderer at :313. So the press that did nothing was aimed wrong again, not wired wrong.
* **the Files view is a stub**, and says so: `files_tab` at :473 draws
  *"This instance's folder is empty."* -- whatever the folder holds.

That is a much better finding than "the tabs are painted, not placed". The page's structure is real;
the **file manager is a placeholder**, and the mod this session installed into
`instances/Instance 2/mods/` would be shown as an empty folder.

So the brief's file-management clause needs a Files view built on the real folder -- listing
`mods/` at minimum, with the reference's columns and row actions -- rather than a control re-wired.

**Also worth changing in how this is measured**: three rounds of this doorway have produced three
different causes, and every one of them was read from a capture rather than from the source. The
source answered in one grep what three captures could not.

### Correction two: the Files tab is implemented and lists the wrong folder (2026-11-01)

`files_tab` (`pages/instance.rs:469`) is **not** a stub. It calls `folder_entries(&instance.path)` and
draws one row per entry with a folder or file glyph and a size, up to twenty-four rows; the "folder is
empty" text appears only when the listing is empty.

The parity bug is in the path it lists: **`instance.path` is the instance root** -- `instance.cfg`,
`mmc-pack.json`, `minecraft/` -- where the reference's Files tab shows the **game directory**, the
folder that contains `mods/`, `config/`, `saves/` and `logs/`. So a reader opening Files here sees the
launcher's bookkeeping beside the game's, and the mod this session installed is one level down inside
`minecraft/` rather than in the list.

**The fix is one path**, and it needs a decision with it: whether `mods/` lives at
`{instance}/mods` (where the installer writes it today) or `{instance}/minecraft/mods` (where the game
reads it). That question is older than this page -- the install proof wrote to `{instance}/mods` and
nothing has yet launched a game to see whether that is the right place.

## The mods path is wrong, and the layout itself is the proof (2026-11-01)

Two lines, read side by side:

    sift-core/src/instance/mod.rs:109-110   ensure_dir(&root.join("minecraft"))?;
                                            ensure_dir(&root.join("jarmods"))?;
    sift-ui/src/install.rs:284              let mods = paths.instances_dir().join(&instance).join("mods");

`Instance::create` builds a **Prism/MultiMC layout**: the instance folder is bookkeeping
(`instance.cfg`, `mmc-pack.json`, `jarmods/`, `libraries/`) and the *game directory* is
`<instance>/minecraft/`. In that layout the game reads `<instance>/minecraft/mods/`, so the installer
writes one level too high, and `files_tab` lists one level too high for the same reason.

**So the defect is one line in two places**, and the proof is the layout the codebase already committed
to -- no launch and no reference capture required.

**The change**: `install.rs:284` -> `...join(&instance).join("minecraft").join("mods")`, and
`files_tab` -> `folder_entries(&instance.path.join("minecraft"))`. Both want the install test that
asserts the written file's path, and the four-install proof re-run afterwards -- which is why they are
recorded and not made in this round: a path change that breaks the only proven flow, shipped unverified,
is the failure this session keeps paying for.

### Both paths moved to the game directory; the re-walk did not land (2026-11-01)

Two edits, tests green:

* `install.rs:284` writes to `<instance>/minecraft/mods`;
* `files_tab` lists `instance.path/minecraft`.

The install proof was then re-walked on the corrected path -- create a Fabric instance, search
`sodium`, press Install -- and **no jar appeared anywhere under `instances/`**. So the sequence did not
complete, and the corrected path is **unverified**: the code now says what the layout says, and nothing
has yet shown a mod landing there.

That is the exact state this session's rules forbid shipping as "done", so it is recorded as open with
its next step named: run the same sequence one step at a time, confirming the page from the frame before
each press, because the last five aim errors were all of that shape and the lab is now empty of
instances -- a fresh create is the first thing to check.

# The path fix is verified: the jar lands in the game directory (2026-11-01)

    instances/Instance 2/minecraft/mods/sodium-fabric-0.9.3-beta.1+mc26.3.jar

That is the corrected location, and it is the fifth install this session has proven with a file -- and
the first one in the folder the game actually reads.

Two failed attempts preceded it, and both failed for the same reason worth recording: **the create
sequence fired more than once**, leaving three instances, and `choose_instance` correctly refused with
"there are N instances and choosing one needs a picker that is not built yet". Trimming to one and
re-running the install alone produced the jar first time.

So the decision the last two rounds were circling is settled by the codebase's own layout, and the
multi-instance refusal -- which has existed since the picker work and had never been exercised in the
product -- is now **also verified**, if only as a refusal.

### The Files press, aimed from the capture, still leaves Content selected (2026-11-01)

Measured off the frame at 1.5x: the Files label spans source x 193..230, its centre y 171, so the
press went to (211,191) on screen. **Content is still the brand-filled tab afterwards.**

That is the seventh press this session that a capture said would land and did not, and the tab bar is
wired in the source (`ui.interact` at `pages/instance.rs:294`). The conclusion is the one the whole
session keeps arriving at from a different direction: **a position read from a picture is a hypothesis**
-- print the tab's rect and press its centre, exactly as the rail and the name field were settled.

**The probe, written out**: in `instance.rs`'s tab loop, print each tab's `rect`, `response.hovered()`
and `response.clicked()` for one press. Three lines, three numbers, and the tab bar stops being a
mystery. Until then the Files tab -- and with it the file-management row -- stays unverified, though its
renderer now points at the right folder and the jar is on disk in the folder it lists.

# The Files tab lists the game directory (2026-11-01)

The probe printed every tab's pill, and one number ended the round:

    tab Files pill=[[170.9,132.0]-[228.2,162.0]]

Centre (199,147) content, (199,167) on screen. Pressed there, **Files is the brand-filled tab and the
panel lists `mods`** -- the folder holding the jar this session installed.

So the chain the brief asks for is now whole on our side: install -> jar in `minecraft/mods` -> Files
tab lists that folder. Two of its three links were proved this round and the last round.

**And the method recorded itself**: seven presses failed reading positions from captures; this one
succeeded reading them from a printed rect, first try. The probe is three lines and the session has
run it five times now, always successfully.

### The Files view lists one level and cannot descend (2026-11-01)

Pressed the `mods` row: nothing changes. The view is a **flat listing** -- rows drawn with a glyph, a
name and a size, no navigation -- so the jar inside `mods/` can be reached on disk and not through the
product.

That is the honest state of the brief's file-management clause: **the folder is listed, the folder
cannot be opened.** The reference's Files view descends into directories, shows a path, and offers
per-row actions; ours stops at the first level.

**The next feature, named rather than guessed**: a path state on the instance page
(`FilesState { path, ... }`), a breadcrumb above the list, and a press on a directory row that replaces
`path` with `path.join(name)`. The renderer already takes any directory -- `folder_entries` is generic
-- so this is navigation, not a second renderer.

### The navigation attempt was reverted; it needs five call sites, not three (2026-11-01)

Started the Files navigation -- `files_dir` on the page state, a `descend` from a pressed folder row --
and the compiler found five consequences the edit had not accounted for:

* `State` derives `Copy`, which a `PathBuf` forbids;
* `files_tab` gains a parameter and a return type, so its call site and its two `return`s change;
* the row loop where the press belongs is a different function from the one the edit landed in.

Reverted wholesale rather than half-fixed, because a tree that does not build is worse than a feature
that is not started. **The scope is now exact**: one state field (and `Copy` off), one signature, one
call site, two returns, one press inside `files_tab`'s own loop. Five places, all named by the compiler
in a single build -- which is the cheapest possible way to learn them.

### The five-edit attempt stopped at an anchor and wrote nothing (2026-11-01)

The second attempt applied its first two edits in memory and then failed to find the third anchor --
the `files_tab` body, whose text had already changed when the path moved to `minecraft` two rounds ago.

Because the script writes once at the end, **the file is untouched**: `git status` shows it clean and the
tests never stopped passing. That is the right failure mode, and it is worth recording as such -- earlier
in this session the same situation produced a half-written file and three repair builds.

**The remaining work is now three edits, not five**, because two of the five were the ones that landed in
memory before the anchor failed and are written out verbatim in the attempt: the `files_dir` field and
the call site that captures the descent. The next session needs the `files_tab` body as it actually is
today -- re-read it rather than recalling it -- and then the row press.

### Files navigation: attempt two, reverted, with the exact tail found (2026-11-01)

Applied the three known edits (state field, call site, signature) and the compiler named the two
remaining consequences at once: `State` must lose `Copy`, and `files_tab` must end with `descend`.

The next script tried to do both and failed on its tail anchor -- the function does not end at
`\n}\n\nfn logs_tab(`, because a doc comment sits between them. **It failed before writing**, so the
file was restored from git and the tree is green again.

So the navigation is now **five edits with four known verbatim**: the state field, the call site, the
signature, and `Copy` off the derive -- all written in this document's previous entries -- plus one that
needs the function's *actual* last two lines, which no attempt has yet printed.

**The single thing the next session should do first**: print `files_tab`'s final five lines, and the
anchor problem disappears.

# Files navigates, and the installed jar is reachable through the product (2026-11-01)

Instance -> Files -> press `mods` -> the panel lists **`sodium-fabric-0.9.3-beta.1+mc26.3.jar`**.

That is the brief's file-management chain working end to end: the mod was installed from Discover into
a Fabric instance, written to `minecraft/mods`, and is now **visible and reachable inside the app** --
the thing that was "on disk and invisible from the product" two rounds ago.

The five edits, all known verbatim before the build:
`files_dir` on the page state; `Copy` off its derive; the call site capturing the descent; the
`files_tab` signature taking and returning a path; and the row press inside `files_tab`'s own loop,
found this time by locating the function's closing brace structurally rather than by guessing its text.

**What the view still is not**: it lists names with a glyph, has no breadcrumb, cannot go back up, and
has no per-row actions. The reference has all four. Those are the next rows, and they are now the only
things between this panel and the brief's file-manager description.

### The folder is no longer a trap (2026-11-01)

One composed capture, two states:

* **descended**: `<- mods` in the brand colour above `sodium-fabric-0.9.3-beta.1+mc26.3.jar`;
* **after pressing that row**: back at the game directory, listing `mods`.

So the Files view now goes in and comes out, and both were verified in the same run rather than inferred
from one direction. The back row is the whole breadcrumb for now -- a single parent step rather than the
reference's clickable path segments -- and it is enough to stop the view being a dead end, which was the
worse defect of the two.

### The remove control is drawn and its press is not yet aimed (2026-11-01)

A file row now carries an `x` at its right edge, and a press there **left the jar in place** -- the
file is intact, which is the important half: no accidental deletion, and the listing still shows
`1.8 MB` beside the name.

The press was aimed from a crop, and the crop shows the size label ending around x 1120 where the
control was drawn at the card's inner right minus twelve -- the eighth press this session to be aimed
from a picture and miss.

**The probe, again three lines**: print `remove` beside the `("file-del", name)` id, press that rect's
centre, then check the folder. Until then the action exists in code, is safe in practice, and is
unverified in behaviour.

# A file can be removed from inside the app (2026-11-01)

Probe: **`remove=[[1071,227]-[1089,245]]`**, centre (1080,236) content. Pressed there, and
`Instance 2/minecraft/mods/` went from one jar to none.

So the Files panel now **lists, descends, returns, and removes** -- four of the five things a file
manager does. The fifth, acting on a file other than deleting it, is still open.

**The overlap hazard the last note worried about does not exist**, and the probe answered it in the same
line it printed: `remove` sits inside the file row, but only *file* rows carry the control and only
*directory* rows carry a press, so no reader can delete a file by trying to open it.

### The round trip closes: install, delete in Files, install again (2026-11-01)

After the delete emptied `mods/`, the Discover install was run once more and the jar is back:

    sodium-fabric-0.9.3-beta.1+mc26.3.jar

So the full cycle works through the product in both directions -- **no stale state, no cached listing,
no instance whose mods folder the app has lost track of.** That was the risk worth checking after adding
a destructive action, and it did not materialise.

**Open, and next**: the delete has no confirmation and no undo. One press removes a jar silently, where
the reference asks first. Two-step arming would fix it -- a press paints the row's control in the
disabled colour and stores its name, a second press on the same row removes -- and it needs one field on
the page state beside `files_dir`, which is now a known shape to add.

# Removal is two-step now, and both steps were measured (2026-11-01)

* **one press**: the file count stays at **1** and the row's control turns brand-green -- armed, not
  destructive;
* **second press**: the count goes to **0**, and the armed state clears.

So the hazard the last round named is closed: a stray press arms, and only a second press on the same row
removes. The armed colour is the only affordance, which is thinner than the reference's modal but is
honest about what the next press will do -- and unlike the modal it costs no state beyond one string.

**What this panel now does**: lists, descends, returns, arms, removes, and reflects changes made outside
it. What it does not do: reveal in the OS, open a file, drag or drop one, or show a file's size
relationship to a limit. Those are the remaining rows.

### Keyboard shortcuts: Ctrl+F does nothing (2026-11-01)

Pressed `ctrl+f` on Discover and typed `sodium`. The field still reads its placeholder -- **"Search
modpacks..."** -- and the listing is unchanged, so the keystrokes went nowhere.

Two facts from one frame, both worth having: **the keyboard-shortcut clause has no implementation on
this page at all**, and the tab state was Modpacks rather than Mods, which is why the result list shows
a modpack. The second is a reminder that even a shortcut test has to establish the page it is on first.

**A shortcut is not hard here** -- egui delivers `ctrl+f` as a key press, and the search field already
has focus logic; what it lacks is someone asking `wants_keyboard_input` before focusing it. That is the
next edit, and unlike drag and drop it can be **driven and verified** in this lab.

### Ctrl+F is wired and the drive shows nothing -- and the drive itself is suspect (2026-11-01)

The shortcut is in the widget: when nothing holds focus and `ctrl+f` is pressed, the search field
requests it. Driven with the pointer parked away from the field and `sodium` typed afterwards, the field
**still shows its placeholder** and the listing is unchanged.

But two other things in the same frame point at the driver rather than the code: the page is still on
**Modpacks**, though the run pressed the Mods pill, and the earlier run in this same shape *did* switch
tabs. So the clicks after the first one may not have been landing either -- which would mean the window
had lost input focus, and `xdotool key` went to whatever did have it.

**The check that settles it costs one command**: `xdotool getactivewindow` before the keystroke, and
`windowfocus` when it is not the app. This session has met that exact failure before -- a click on an
unfocused window only focuses it -- and the note is here so the next attempt does not read a driver
problem as a code problem, or the reverse.

### Settled: the window had focus, so the shortcut itself is what fails (2026-11-01)

`xdotool getactivewindow getwindowname` answers **"Sift Launcher"**. So the keyboard was going to the
right window, and the Ctrl+F that did nothing is a **code** failure, not a driver one.

That also means the two clicks in the same run that did not switch tabs were separate misses, which is
consistent with everything else this session has recorded about aiming.

**The next probe is one line**: alongside the `ctrl+f` test, print
`ui.input(|i| (i.modifiers.command, i.key_pressed(egui::Key::F), i.focused))`. If `command` is false,
xdotool's ctrl is not arriving as a modifier; if the key press is false, the event is being consumed
before this widget sees it; if both are true, the focus request is what fails -- and each of those three
has a different one-line fix.

# Ctrl+F: the key arrives, the field ignores the focus (2026-11-01)

The probe answered all three questions in one line:

    key probe: command=true keyF=true focused=true none_focused=true

So the modifier arrives, the key press arrives, the window has focus, and nothing else holds egui's
keyboard -- **the shortcut's own path is correct**. Pressing Ctrl+F does call `request_focus` on the
search field.

What fails is the field. It is hand-drawn (`widgets/search.rs`), and its text handling evidently keys off
something other than `has_focus` -- most likely the `response` it gets from `interact` earlier in the
same frame, which cannot see a focus granted after it. So the fix is in the field's text path, not in
the shortcut, and the shortcut can stay as written.

**This is the round's real result**: three candidate causes, one probe, and the one that was left is the
only one that needs a code change. The probe cost one build and removed two of the three.

### Setting the field's own flag is not enough (2026-11-01)

The shortcut now sets `self.focused = true` as well as requesting egui's focus, and the drive is **still
negative**: placeholder showing, no text, listing unchanged.

So the condition at `widgets/search.rs:103` is not simply `self.focused`. Something earlier in the same
frame clears it or gates it -- line 100 sets `self.focused = false` on some other event, and the run
presses a tab pill immediately before the shortcut, which is exactly the kind of press that would.

**The probe is now one line at the text path**: print `self.focused` and `i.events.len()` at line 103 and
inside the loop. Two rounds of guessing at this widget have produced two negative drives; the third
attempt should print rather than reason.

**Recorded as owed**: Ctrl+F does not work, the code that tries to make it work is in the tree, and it is
marked here rather than described as finished.

### Ctrl+F: three fixes, three negative drives -- stop guessing (2026-11-01)

The guard was removed (it was a plausible blocker: every click gives its widget focus, so "nothing is
focused" was false after any press), and the drive is **still negative**. The tab is also still
**Modpacks**, though the run presses the Mods pill -- which means the pill press is *also* not landing.

Two failures in one run, and the second one explains why the first is unreadable: **the drive is not
established, so the result cannot be attributed to the shortcut at all.**

So the next session should treat Ctrl+F as **unimplemented and undiagnosed**, and start with the thing
this document has said eleven times: **probe the pill's rect and the field's rect, press their centres,
and only then judge the shortcut.** Three speculative edits have gone into this widget; none has been
tested against a drive that was known to be working, which is the mistake, not the edits.

# CLOSED: Ctrl+F focuses search, typed without touching the field (2026-11-01)

    Discover -> Mods (pill at screen y 154, not 120) -> ctrl+f -> "sodium" typed -> results

The field holds **sodium** and the listing answers with Sodium Plus and the rest, and **the pointer never
entered the field**. So the shortcut works end to end.

It needed both edits and a correct drive, and the drive was the part that had been wrong for three
rounds: **the Mods pill sits at screen y 154**, and every attempt had pressed 120 -- thirty-four points
above it, into the tab row's empty margin. With the page still on Modpacks, the search field was the
*modpack* field, and a query of "sodium" would have produced nothing to look at even when the focus
worked.

Both edits stay: setting the widget's own `focused` flag (it does not read egui's memory focus) and
removing the "nothing is focused" guard (every click gives its widget focus, so that condition was false
after any press). Either alone would have failed.

**The lesson, for the third time in this project and the first time about a click**: the shortcut was
never certainly broken; the pointer was.

### The pager works, and the arrow is at x 1011 (2026-11-01)

Pressed the next-page arrow: **page 2 becomes the brand-filled pill**, where page 1 was. So Discover's
pager is real, wired, and moves the listing -- a row that had been "drawn" in this document since the
first page sweep and had never been driven.

The first press missed: aimed at (923,205) from a 0.655-scaled crop, while the arrow sits at
(1011,240). **The twelfth aim error, and the same arithmetic as the eleventh** -- a coordinate read from
a resized image, off by the scale factor.

**Where this leaves the Discover page**: search (typed, results, facets), tabs (Mods pressed and
effective), the pager (page advance), and the Ctrl+F shortcut. Four behaviours, each verified by driving.
Not yet: sort dropdown, view-count dropdown, category checkboxes, the install picker modal.

### The sort menu opens, with the reference's five options (2026-11-01)

Press at (220,240) -- the sort control's centre, computed with the crop's scale factor this time --
opens a menu: **Relevance** (brand-filled, the current value), **Downloads**, **Follows**, **Newest**,
**Updated**.

So this is not a painted control: it is a real dropdown with a real selection state. **What is not yet
shown is that choosing an item changes the ordering** -- the menu opened, nothing was pressed inside it.

That distinction is the same one the matrix keeps drawing: *opens* is one row, *acts* is another, and
this round verified the first.

### Choosing from the sort menu changes the value (2026-11-01)

The press was aimed at **Downloads** and landed on **Follows** -- one row lower, the thirteenth aim error
-- but the result is unambiguous: the control now reads **"Sort by: Follows"** where it read
"Relevance", and the listing redrew with page 1 of 956 again.

So the sort dropdown is **open-and-select**, not open-only: state changes, the request is re-issued, the
pager resets. Whether the *ordering* differs between Relevance and Follows for this query is a separate
question the frame cannot answer, because both put Fabulously Optimized first.

**Rows verified this stretch, all by driving**: search, tabs, pager, Ctrl+F, sort menu, sort selection.
Six behaviours on one page, where the matrix had one row.

### The view-count dropdown opens too (2026-11-01)

First press at (545,240) missed -- the control ends near x 495, so that was past its right edge, the
fourteenth aim error. Pressed at (455,240) and a **dropdown opens** with its current value brand-filled
among four items.

So both selects on Discover's toolbar are real: **sort** (opens and selects) and **view count** (opens).
Choosing a count is the next row, and the routine is the one that has just worked twice: press the
control at a computed centre, then press an item at a computed centre.

### The category list is labels, not filters (2026-11-01)

Pressed "Adventure" in the sidebar and the frame either side is identical: **no check, no redraw, no
change of listing.**

And the crop says why in one look: **the categories have no checkboxes at all.** The reference's
sidebar draws a control beside each category and filtering by it; ours draws a word. "Hide already
installed" above it *does* have a switch -- so the sidebar is not uniformly unfinished, it is unfinished
exactly where filtering lives.

**Named next**: a control per category (the reference's checkbox), a selected set on the page state, and
the category facets added to the request the way the search text already is. The facet machinery exists
-- the mod card's tag chips come from the same vocabulary -- so this is a UI gap rather than a data one.

### The sidebar lives in panel.rs, and it models the categories (2026-11-01)

One grep for the label found the file this document had been calling "the sidebar":

    crates/sift-ui/src/panel.rs:66   const SEE_MORE: &str = "See more";
    crates/sift-ui/src/panel.rs:68   const HIDE_INSTALLED: &str = "Hide already installed";
    crates/sift-ui/src/panel.rs:71   One category row's height ...
    crates/sift-ui/src/panel.rs:259  The list's See more / See less.

So the panel **models** the category list and its expand/collapse; what it does not model is a *selection*
-- no checked set, no facet in the request. That is a smaller gap than the last note implied: the rows,
their heights, the see-more boundary and the installed switch all exist, and the missing piece is the
state behind them plus the facet string.

**Next**: a checked set on the panel's state, a press per row, and the facet appended where
`search_url_parts_with_facets` already takes a list -- named at `sift-net/src/modrinth.rs:46`.

### Correction: the category facets exist (2026-11-01)

Reading `panel.rs` overturns the last note's central claim:

    panel.rs:584   CATEGORY_ROWS.min(categories.len())
    panel.rs:599   let chosen = facet.is_some_and(|chosen| chosen == format!("categories:{name}"));
    panel.rs:602   pressed = Some(Step::Category(format!("categories:{name}")));

So the panel **takes the categories from the API**, **knows which facet is chosen**, **emits a Step when
one is pressed**, and has a `See more` that reveals the rest. That is not "labels where the reference has
filters" -- it is the whole mechanism, with the press missing its target or the Step not being applied by
the caller.

**This is the fourth time in this project that a capture said "unbuilt" and the source said "unwired",**
and the pattern is now worth stating as a rule rather than a lesson: *a press that does nothing is
evidence about the press, not about the feature.*

**Next**: read the caller's handling of `Step::Category` -- if it appends the facet to the request, then
the only defect is the row's hit rect, and the probe is one print of it.

### The category filter is complete end to end; only my aim was wrong (2026-11-01)

The caller:

    shell.rs:1592  Step::Category(facet) => {
                       self.facet = if self.facet == Some(facet) { None } else { Some(facet) };
                       self.page = 0;
                       self.results.forget();
                   }

So a press toggles the facet off and on, resets to page one and drops the cached results -- the exact
behaviour the reference has. **The whole chain exists**: the panel lists categories from the API, marks
the chosen one, emits the Step; the shell toggles, resets and refetches; the request already takes facets.

**There is no feature to build.** There is one press to aim, and the probe is one print of the category
row's rect -- the same three lines that settled the rail, the name field, the card, the tabs, the sort
control and the file rows.

**What this says about the matrix**: at least one "OPEN" row is a working feature that a capture
misjudged twice. The next re-count should treat rows whose only evidence is a failed press as
**unverified rather than open**, because four of them have now turned out to be complete.

# CLOSED: the category filter works when the row is pressed at its rect (2026-11-01)

Probe:

    category adventure row=[[1156,372]-[1424,398]]  pointer=Some([1199,378])  clicked=false

Pressing at content (1199,385), screen (1199,405), turns **Adventure brand-green**: the facet is set, the
row renders chosen, and the shell has already been read to reset the page and drop its result cache.

So the whole feature -- list, mark, toggle, refetch -- is verified end to end. **It was never missing.**
Three rounds called it absent (labels, no checkboxes, no filter) and all three were wrong in the same
way: each judged it from a press that missed the row by 12 to 34 points.

**The count this changes**: "OPEN 512" contains at least one row that this round closes and four more
that are known-complete, so the real figure is lower by an unknown amount that a re-count would supply.

**Two rules, both earned**: print the rect before pressing it; and when a capture says a feature is
missing, read the source before believing the capture.

### The hide-installed switch: pressed, not yet read (2026-11-01)

Pressed at (1410,353) and the frame either side is **inconclusive** -- the crop I chose cut the control
across its top edge, so the knob cannot be compared.

That is the fifteenth aim error and the first one of the *reading* rather than the pressing: the press may
have landed, and the evidence cannot say. **The fix is the same as always** -- crop the control whole, or
better, print the switch's rect and the boolean behind it, which is a stricter test than a picture of a
knob in either state.

**Not claimed**: whether the switch filters installed projects. The row stays open and its evidence is
recorded as a bad crop rather than a negative result, because those are different things and only one of
them is information about the product.

# CLOSED: the instance card's context menu opens (2026-11-01)

    kebab Instance 2 rect=[[1080,154]-[1108,182]]   centre content (1094,168), screen (1094,188)

Pressed there, the menu opens with the reference's items: **Play**, **Edit**, **Duplicate instance**,
**Open folder**, and **Delete** in the danger colour, each with its own glyph.

**So the brief's context-menu clause has had a working implementation all along**, and this document has
called it "painted, not placed" twice -- on the strength of a press aimed at (1063,216), which is
**twenty-eight points below and thirty-one to the left** of the button.

That is the fifth feature this stretch found complete and merely unreached, after the Files panel, the
tab bar, the card body and the category filter. **Five for five: every "unbuilt" claim that was checked
against the source turned out to be a press that missed.**

**Still unverified**: what each item does beyond Edit (which was already driven, opening the instance
page). Play, Duplicate, Open folder and Delete are drawn and modelled; none has been pressed.

## Skins and Screenshots audited against source (2026-11-01)

Full report: `.scratch/reports/page-skins-screenshots.md` -- 44 rows, one table, every claim citing the reference's own file and line.

**Five P0s, in the order they matter:**

1. **The skins page invents a toolbar.** A search field, Wide/Slim tabs and an Add-skin button, none of
   which exist in `Skins.vue:1104-1106` -- the reference's upload is the **grid's first tile**
   (`VirtualSkinSectionList.vue:426-443`). An invented control is worse than a missing one: it teaches a
   reader a layout the product does not have.
2. **No rotatable 3D preview.** `skins.rs:1424-1433` blits a 2D atlas; the reference drags at
   0.01 rad/px (`use-skin-preview-controls.ts:24-30`), and there is no nametag, no `Previewing` badge and
   no Apply/Reset pair.
3. **The preview column is 320px + 16px gap** where the source is `1fr:2.5fr` with a 40px gutter --
   three columns at 1440 where the reference has four.
4. **Skin cards carry name/author/model text** and never show the selected check or the hover
   edit/delete overlay (`SkinButton.vue:90-96,118-123`).
5. **Screenshots**: the hover caption is drawn permanently (`card.vue:233` is `opacity-0` until hover),
   there is an invented page-level count pill, and **select mode, context menu and delete do not exist**.

**A warning worth keeping**: `skins.rs:89-97`'s module docs are stale -- they claim there is no preview
column and no shell wiring, both of which are false. **Documentation inside the source can lie as easily
as a capture can**; the code below it is what counts.

**Named first fix**: P0 1, because deleting an invented toolbar is a removal rather than a build, and it
makes the page honest before anything is added to it.

### The audit's top P0 collides with a documented decision (2026-11-01)

Checked `skins.rs` before deleting the "invented" toolbar, and its own header says it is not an
accident:

    skins.rs:41  has no search field, no model tabs and no `Upload` button in a toolbar: the only upload
    skins.rs:47  The toolbar, the two empty states and the author line are therefore this launcher's ...

So the toolbar is a **recorded deviation**, not an invention -- the same category as the two rows this
document already marks DELIBERATE. The subagent read the pixels against `Skins.vue` and was right about
the difference; it did not have the code's own note saying the difference was chosen.

**That changes the fix from "delete it" to "decide it"**: either the deviation is right and the row is
DELIBERATE with its reason, or it is wrong and the toolbar goes. **What it must not be is a P0 defect and
a documented decision at the same time**, which is what the last two entries made it.

**And the method lesson repeats, one layer up**: the source corrected the capture last round; this round
the source's *comments* corrected an audit. Neither a picture nor a report is evidence about intent --
only the code and its notes are.

## Servers, Project and About audited against source (2026-11-01)

Full report: `.scratch/reports/page-servers-project-about.md` -- 57 rows (16 P0 / 21 P1 / 20 P2) across
Servers (25), Project (23) and About (9).

**Five P0s:**

1. **The servers search field is inert.** `servers.rs:1514-1555` paints the plate and `State::search` is
   never written (`:235`, `:260`); the reference filters name, loader and version as you type.
   *This is the same defect as the one this session fixed on Discover* -- a field that takes no text --
   and the fix is the same shape: give the field its own focus flag and write the literal.
2. **Server cards are not targets.** No click, no keyboard, no hover or active state
   (`servers.rs:1595-1654`), where the reference's `ServerListing.vue` is a link with both.
3. **The servers page is inset twice** -- heading at x=112 where it should be 88, `PAGE_PADDING` applied to
   an already-inset rect -- and its controls sit on a second row where the reference puts them beside the
   `h1`.
4. **Project header scale is wrong**: 64/20/13 pt against the reference's 96 pt avatar, `text-2xl`
   semibold title and 16 pt summary at `max-w-[44rem]`.
5. **Project install button and kebab are the wrong shape and size** (112x40 r8 against `h-12
   rounded-2xl` and a `w-12` menu), **and the Description carries an invented "How to install / Discord /
   Rate" row** (`project.rs:335-368`) that the reference does not have.

**Two structural findings, both bigger than any row:**

* on a project page the reference **hides the default sidebar**; we append the shared stack, so the panel
  is drawn where the reference draws nothing;
* `ServersPromo` is unused, and the sidebar promo is a **300x250 image** where our panel paints a 136 pt
  card **on every page**.

**And the discipline worth copying**: the report marks UNKNOWN where it must -- About has no reference page
at all (no `About.vue`, no `app.about.*` keys), MOTD has no reference element, and **no Servers, project
or About capture exists in `.scratch/cap/`** beyond `ref-home.png`, so every pixel-only question is
flagged UNKNOWN with the exact capture that would settle it. That is the behaviour this document has been
asking for since its fourth retraction.

### Checked the servers search against the source: the audit is right (2026-11-01)

`search_plate` is a **pure painter**:

    fn search_plate(painter: &egui::Painter, rect: Rect, theme: &Theme, count: usize)

No `interact`, no state, no event loop -- it draws a rounded plate, a glyph and a placeholder. So the
audit's first P0 holds: **the servers search field cannot take text**, and unlike the category filter and
the card menu there is nothing to uncover here. It is genuinely unbuilt.

That makes it the **first audit claim checked and confirmed**, after two that were corrected (the skins
toolbar, which is a documented deviation). Both outcomes were useful, and the difference between them was
one read of the source.

**The fix is now well-known**, because Discover's field already has it: a `focused` flag on the servers
state, an `interact` with `Sense::click_and_drag`, an event loop writing the literal, and the filter
applied where the list is built.

### The servers search: one location, and it is the wrong one for input (2026-11-01)

`State::search` **already exists** and the page **already filters by it** ("this page filters by it and
counts the placeholder with it"). What is missing is only a widget that *writes* it.

Tried to wire that at `servers.rs:1494` (`search_plate(painter, search, theme, shown)`) and the compiler
refused: **that function has a painter, not a `Ui` and not the state.** So the plate is drawn from inside a
painting pass, and the press has to be registered wherever the page *does* have a `Ui` -- one frame up.

Reverted, tree green. **The next attempt starts one function higher**, and the work is then the same
four lines used on Discover: interact, focus flag, event loop, and the filter that already exists.

### The search rect is returned to the caller -- one read left (2026-11-01)

`heading_and_toolbar`'s signature is the answer the last round was looking for:

    fn heading_and_toolbar(painter, rect, theme, count, shown) -> (f32, Rect)

**It returns the search field's rect.** So the drawing pass already computes the geometry, and its caller --
which owns the `Ui` -- can register a press on that rect and write `state.search`. No new geometry, no
second painter function: one `interact` at the call site plus the event loop Discover already uses.

Tried to patch the call site by pattern and **the destructuring did not match**, so nothing was written and
the tree stayed green. **The one unknown is now a single line of someone else's code** -- the exact form of
the `let (...) = heading_and_toolbar(...)` binding -- and after that the change is four lines.

**This is the whole recipe, written out so it does not have to be rediscovered**: read the binding, add
`state.search_focused`, interact on the returned rect, loop over `Event::Text`/`Backspace`/`Escape` into
`state.search`, and the filter already runs.

### Correction: the returned rect is the New-server button, not the search field (2026-11-01)

The call site, read verbatim:

    let (list_top, new_server) = {
        let painter = ui.painter();
        heading_and_toolbar(painter, rect, theme, state.servers.len(), shown)
    };

So the tuple's `Rect` is **`new_server`** -- the button the next lines press -- and **the search plate's rect is
computed inside and never returned**. Last round's note said the opposite, and it was wrong: I read the
signature and assumed which `Rect` it carried.

**The change is therefore three edits, not four lines**: have `heading_and_toolbar` return the search rect
alongside the others, bind it at this call site, and register the press there -- the caller has `ui`, which
is exactly what the plate's own function lacks.

**And one detail from the same lines that matters more than the geometry**: this page reads presses from
the pointer rather than registering them with egui, "because a full-column interaction registered later in
the frame would take the press away". **Whatever wires this field has to respect that ordering**, or it will
break the New-server press it sits beside.

# CLOSED: the servers search takes text and filters (2026-11-01)

Pressed the plate, typed `hyp`, and the page answered:

    Search 0 servers...      (was: Search 6 servers...)
    No servers you own match your search.

So the field accepts input, `State::search` is written, and the filter that already existed now runs. The
audit's first P0 is closed.

**How it was done**: `heading_and_toolbar` now also returns the plate's rect (`-> (f32, Rect, Rect)`), the
caller binds it, and the press plus the event loop sit **before** the New-server plate's press -- the
ordering this file warns about, honoured rather than rediscovered.

**What is still wrong, and it is visible in the same frame**: the field paints its **placeholder** where
typed text should be. The literal goes into the filter but never onto the screen, so a reader sees
"Search 0 servers..." and no query. That is the next cosmetic row, and it is one line in `search_plate`:
draw the text instead of the placeholder when it is non-empty.

### The field shows what was typed (2026-11-01)

Typed `hyp` and the plate now reads **"hyp"** where it read "Search 0 servers...". Five edits, all
mechanical: the text reaches `search_plate` through `heading_and_toolbar`, and the plate draws it in place
of the placeholder when it is non-empty.

**So the servers search is complete in the three parts that matter**: it takes the keyboard, it shows the
query, and it filters the list -- each verified by driving in the last two rounds.

**What it still is not**: a text field with a caret, a clear button, or a cursor; the reference's Input has
all three. The plate draws the literal at the placeholder's position, which is honest but not identical --
and the difference is now visible side by side rather than invisible, which is the right order to fix
things in.

# The count, with UNKNOWN included (2026-11-01, round 161)

| status | rows |
| --- | --- |
| DONE | 93 |
| OPEN | 512 |
| BLOCKED-SERVICE | 9 |
| RENDERER-RISK | 16 |
| NOT-IN-SOURCE | 13 |
| DELIBERATE | 2 |
| UNKNOWN | 23 |
| **total status-bearing rows** | **633** |

**Read this table with two things in mind.**

First, the **101 audit rows** from the two subagent reports (Skins/Screenshots, Servers/Project/About) are
cited in the prose above but are **not** table rows, so they are not counted here: the real open surface is
about **613**, not 512.

Second, **93 DONE is a floor, not a total**. Five features this stretch were recorded as unfinished and
turned out to be complete -- the Files panel, the tab bar, the card body, the category filter and the card's
context menu -- because the presses that judged them had missed. Every one was corrected by reading the
source. **An unknown number of the 512 OPEN rows are in the same state**, and the only way to find out is
the same read-then-press sequence that found those five.

**So the matrix's headline is unreliable in both directions and should never be quoted alone again**: it
overstates progress where rows were closed on a capture, and understates it wherever a press missed.


### Server cards: the plan, and the anchor that failed (2026-11-01)

`draw_section` (`servers.rs:1627`) already computes every card's rect:

```rust
let card = Rect::from_min_size(
    egui::pos2(left, list_top + index as f32 * (card_height() + LIST_GAP)),
    egui::vec2(width, card_height()),
);
```

and then discards it into `draw_card`. **The change is to return those rects** -- `-> Vec<Rect>`, pushed in
the loop, `return cards` in the empty branch -- so the caller, which owns the `Ui`, can make them targets.
**A returned `Vec` that is ignored compiles cleanly**, so the signature change can land on its own before
the caller uses it: two commits, neither of which can break the other.

The edit was attempted and **stopped at the third anchor** -- the exact whitespace around
`draw_card(painter, card, theme, server, &shown, icon);` and the closing brace differs from what `awk`
printed with its leading offset. Nothing was written; the tree stayed clean.

**Next attempt**: read `servers.rs:1665-1672` with `sed -n` (no offset), edit those five lines by hand, and
the rest follows the search-field recipe exactly.

### First of two commits: `draw_section` now hands back the card rects (2026-11-01)

`draw_section` returns `Vec<Rect>` -- one per card, pushed where the card is drawn -- and its empty branch
returns an empty vec. The caller ignores it for now, which is exactly why this could land on its own: **a
dropped `Vec` is not an error**, so the geometry move and the interaction that will use it are two commits
that cannot break each other.

**The line-based edit is what made it work** after two failed attempts at text anchors: instead of matching
whitespace, it found the function by its `fn` line, walked to the closing brace, inserted the vec and the
push by index, renamed the early `return;` and spliced the tail. **No anchor, no offset, nothing to guess.**

**The second commit** is the caller's: bind the returned vec and register an `interact` per rect, ordered
before any other press handling in that frame -- the ordering this file warns about, and the one the
searches honoured.

### The second half needs a visible effect, or it is dead code (2026-11-01)

The rects exist and are ignored. Wiring only the `interact` would make the app *report* a press nobody can
see -- which is the same failure this session spent a hundred rounds unlearning, so it was not landed.

**What a press should do, decided now rather than during the edit**: select the card, because the reference
opens the server's detail view from it and there is no detail view here yet. So the slice is four edits,
not one:

1. `State::selected: Option<String>` (the server's address), and its initialiser.
2. `draw_section` takes `selected: Option<&str>` and passes a `chosen: bool` into `draw_card`.
3. `draw_card` paints the chosen state -- the reference's card border and background shift on selection.
4. The caller binds the returned rects, interacts per card, and sets `State::selected`.

**Then it is drivable**: press a card, capture, and see the border change; press another, see it move.

**Left unlanded rather than half-landed**, because a selection state with no visible difference is
indistinguishable from no selection at all -- and this document has four retractions saying so.

# CLOSED: server cards are targets, and selection is visible (2026-11-01)

Pressed a card and the **Modrinth SMP** row grew a brand-green bar on its leading edge that was not there
before -- and the card above it did not. So the audit's second P0 is closed: cards respond to the pointer,
and the response is visible rather than merely recorded.

**Three edits, all landed at once because the geometry already existed**: `State::selected`, the initialiser,
and the caller binding the returned rects to interact, set the selection and paint the bar.

**And one deliberate simplification worth naming**: the reference opens a *detail view* on a card press and
this launcher has none, so the press selects instead. That is a smaller behaviour, honestly chosen, and it
is drivable -- which the "open a detail view" version was not, without building a whole page.

**Two audit rows closed, both driven**: the servers search (types, shows, filters) and now the cards.

### The selection can be taken back (2026-11-01)

Two presses on the same card and the bar is gone; one press and it appears -- the two halves were captured in
consecutive rounds. So the highlight is a **toggle**, not a one-way state a reader cannot undo.

**Why it matters more than it looks**: the last round's risk was a highlight that does nothing, which is its
own kind of lie. A toggle is not the reference's behaviour -- the reference opens a detail view -- but it is a
**complete, reversible, honest** behaviour, and it is the correct placeholder until that view is built.

**Recorded as the honest gap**: pressing a card in the reference goes somewhere; here it selects. The row
stays open on that basis rather than being closed on a bar.

### The detail view: scoped to the smallest thing that is not a placeholder (2026-11-01)

The reference opens `/servers/:id` on a card press; this launcher has no such page, and the selection bar is
the honest stand-in. Before building a page, note what already exists to fill one:

* `State::live` holds **a live status per server** -- player count and max, already fetched and already
  printed on the cards ("441/5000");
* `State::selected` names the server;
* every `Server` carries address, name, loader, version and MOTD, all already rendered on the card.

**So a detail view is a second rendering of data that is on screen already.** The smallest honest version is
a panel that replaces the sidebar promo while a server is selected, showing the name, the loader and version,
the MOTD, the live count, and one action ("Join"). It needs no new fetching and no new state beyond what
`selected` holds.

**What it is not**: a route. The reference's is a page with its own URL and its own loading state; a panel is
a smaller thing and should be recorded as such rather than as parity.

### The selected server now renders where the promo sits -- unobserved (2026-11-01)

The servers page draws the chosen card's server into the `promo` rect (`servers.rs:1342`), and the promo
only when nothing is chosen. It compiles and the tests pass.

**It was not seen.** The capture I compared cropped the *shell's* sidebar -- "Sync settings across
instances" -- which is a different card from the servers page's own promo, and the two frames were
identical because I was looking at the wrong rectangle.

So this is **landed and unverified**, and the note says so rather than describing it as working. The next
check is one capture of the servers page's own promo area before and after a card press.

**The mistake is the fifteenth of its kind and the first about where to look rather than where to press**:
the address of the thing to compare is as much a hypothesis as the coordinate of the thing to click.

### The detail panel draws at y 1140 -- off the screen (2026-11-01)

The probe answered both questions at once:

    promo rect=[[112,1140]-[1092,1245]]  selected=Some("play.cubecraft.net")

**Selection works** -- the state holds the pressed card's address. And **the panel is drawn a thousand pixels
down the page, below the 900-pixel window**, so it cannot be seen without scrolling. That is why two captures
showed nothing: the thing I built renders off-screen, not badly.

**A second finding falls out of the same numbers**: the promo spans x 112..1092, the *main column*, not a
sidebar. The reference's `ServersPromo` is a sidebar card; ours is a full-width block under the lists. The
row is open for that reason, and it is a layout gap rather than a missing component.

**So the detail panel needs a home that is on screen** -- the natural one is directly under the toolbar, above
"Your servers", where the reference's page-level content begins.

### Off-screen again, and the reason is now a single word (2026-11-01)

Moved the panel to `area.bottom() - DETAIL_HEIGHT` and captured the whole frame: the **card selection bar is
clearly visible** on "All the Mods 10" -- so that half of the interaction is confirmed by a full-frame capture
for the first time -- and **the panel is still absent**.

The cause is in the geometry, not the drawing: **`area` is the scrollable content rect, not the viewport.**
Its bottom is past the window whenever the list is longer than the screen, which it is here (eight cards).
Anything placed relative to `area` inherits that.

**So the fix is one word**: place the panel relative to `rect` -- the page's own rect, which *is* the
viewport -- not `area`. That is the third attempt at this panel and the first where the reason is exact
rather than suspected.

### Stop: the panel is built and cannot be placed without knowing two rects (2026-11-01)

Fifth attempt, `rect.bottom()` this time, and the foot of the page is still the card list. **So `rect` is
content-relative too** -- the page function is handed the scrollable rect, and neither `area` nor `rect` is
the viewport.

**Five rounds on one panel is enough to stop and write down the one measurement that ends it**: print
`rect`, `area`, and `ui.clip_rect()` together for one frame at 1440x900. The third is the viewport, and if it
is what it says it is, the panel anchors to that and appears. **No further guessing.**

**What is genuinely finished here**: the card **is** a target, the **selection is visible** (captured on the
full frame), and the selection **toggles**. What is not finished is the panel that gives the selection a
consequence -- it is drawn, and it is drawn off-screen.

**Recorded as the honest state**: a bar that can be toggled and goes nowhere. That is better than no
interaction and worse than the reference, and it is written down as the second rather than the first.

# CLOSED: the selected server's panel is visible (2026-11-01)

The foot of the servers page now shows a plate reading **All the Mods 10** over **play.cubecraft.net** --
the selected card's name and address, on screen, with real data.

**The cause was paint order, not geometry.** The panel was drawn before the cards, on the panel's own layer,
so the cards painted over it every frame. The measurement settled the geometry in one line:

    rect=[[88,48]-[1116,900]]   area=[[112,172]-[1092,1245]]   clip=[[0,172]-[1440,876]]

`rect` was already the viewport. What was wrong was that the panel sat *under* the list. Painting it on a
**foreground layer** -- the lesson the create dialog taught this codebase two hundred rounds ago -- made it
appear without moving a pixel.

**Six rounds on one panel, and the last five were spent looking for a geometry bug that did not exist.** The
real defect was the same one already written in this document, under a different page: *a plate drawn on a
lower layer is covered by whatever is painted after it.*

### The panel's third line needs `Shown`, not a string (2026-11-01)

Tried to add the live player count with `shown_of(server, &state.live)` and the compiler answered:
**`Shown: ToString` is not satisfied** -- it is a struct, not a formatted string, and the card must print its
fields.

Reverted; tree green. **The detail panel stays as it was verified last round**: name and address, both real,
both on screen. The count is one field away and the next attempt should read `fn shown_of`'s return type
before reaching for it -- which is the same rule that has cost this session the most and keeps costing it:
read the signature, then write the call.

# The server panel carries live data (2026-11-01)

    All the Mods 10
    play.cubecraft.net
    NeoForge 21.1.72 CubeCraft  ·  480/5000 online

Three lines, all real: the name from the card, the address, and the **live status** -- loader, version and the
current player count, which the page was already fetching for the cards and printing nowhere else.

**The read-then-write rule paid off in the same call it was adopted**: `Shown`'s fields were grepped and the
formatting written against them, so the compiler had nothing to refuse. **That is the first time in this
stretch that no revert was needed**, and it is the direct difference between this round and the last three.

**What the panel is**: a second rendering of data already on screen, which is a smaller thing than the
reference's server page and should be recorded as such. **What it is not**: a route, a join action, or a MOTD
block -- the summary is still the card's, not a description.

### The panel's one action copies the address -- label verified, clipboard not (2026-11-01)

A "Copy IP" button sits on the panel's right; pressed, it reads **"Copied"**. So the press lands, the state
changes and the label follows.

**The clipboard itself is unverified**: the lab has no `xclip`, so nothing could read the selection back.
The call is `ui.ctx().copy_text(server.address.clone())`, which is egui's own clipboard path and the same one
the rest of the app uses -- but "the same code as elsewhere" is an argument, not a capture, and this document
records it as an argument.

**Why copy rather than join**: the reference's panel has a Join that launches the game; this launcher has no
server-join path at all, and **a button that launched nothing would be a button that lies**. Copying is the
one action the page can honestly offer, and it is genuinely useful to a reader pasting an address.

### Smoke test after the day's changes: every rail page, no panic (2026-11-01)

Clicked through the whole rail -- Home, Discover, Skins, Screenshots, Servers, plus, Settings, About -- after
every change made today, and:

* the process was **alive** at the end;
* the log contains **zero** occurrences of "panic" or "panicked";
* the frame shows the **About dialog** open with `Sift Launcher 0.1.62`, the copyright line and the
  third-party notices table (Sift Launcher, `itar`, Lucide Icons, egui `eframe`).

That is the cheapest regression check this project has, and it is worth running after any multi-file day: one
launch, one click per page, one `grep -c panic`, one frame.

# The clipboard action is verified: the address is really on the clipboard (2026-11-01)

Installed `xclip` in the lab (recorded as drift below), pressed **Copy IP**, then read the selection back:

    $ DISPLAY=:99 xclip -selection clipboard -o
    play.cubecraft.net

So the button does what it says. **The last unverified claim in the servers page is now verified**, and it took
one package and one command -- which is the honest measure of how much the previous round left hanging.

**Lab drift, recorded**: `xclip` is installed in `sift-lab` by hand, exactly like openbox before it. Neither
is in the committed Dockerfile, so a fresh container needs both before input or clipboard checks will work:

    pacman -S --noconfirm xclip openbox

### Decision: the skins toolbar is DELIBERATE, on a condition (2026-11-01)

The audit called it an invented toolbar and the source calls it a chosen divergence. **Both are right about
the facts and the difference is intent**, so it is decided here rather than left as both:

**Status: DELIBERATE.** The toolbar stands as the launcher's own upload affordance *because the reference's
real one -- the grid's first tile -- is not built yet*. A page with no way to add a skin would be worse than
a page with one in the wrong place.

**The condition that ends it**: when the grid's first tile uploads a skin, the toolbar goes. That is written as
the criterion rather than as a wish, so the row cannot quietly become permanent:

| element | status | evidence |
| --- | --- | --- |
| Skins toolbar (search, Wide/Slim, Add skin) | DELIBERATE until the grid's first tile uploads | `Skins.vue:1104-1106` has no toolbar; `VirtualSkinSectionList.vue:426-443` is the real upload tile; `skins.rs:41-47` records the choice |

**What this does not excuse**: the search field inside that toolbar still does nothing, and a control that
does nothing is not covered by "deliberate". The toolbar may stay; its inert field may not.

### The skins search filters, and still shows its placeholder (2026-11-01)

Typed zzz into the skins field: **the Saved skins grid emptied**, so the query reaches the filter and the
page filters by it. The plate still reads "Search skins...", so the display half is missing -- exactly the
state the servers field was in for one round before its literal was drawn.

**Three edits, no revert, and the reason is now routine**: this page's toolbar function already takes a Ui
AND the state, so the press and the event loop belong in it directly -- no rect to return, no caller to
change. **The cheapest form of the recipe this project has found.**

**Next, one line**: pass the query into search_plate and draw it when non-empty, as servers.rs does.

### The skins plate now receives the query, and does not draw it yet (2026-11-01)

The signature takes `text: &str` and the call passes `&state.query`; inside, the placeholder is still drawn
unconditionally. That is the same two-commit split that worked for the servers panel and the card rects: **the
plumbing lands first and compiles, then the drawing lands against it.**

The literal sits in a galley built near the end of `search_plate`; drawing it instead of the placeholder when
`text` is non-empty is the next one-line change.

# CLOSED: the skins search types, shows and filters (2026-11-01)

The field reads **zzz** after typing it, and the Saved skins grid emptied when it did -- so the three parts
hold on this page as they do on the servers page: **takes the keyboard, shows the query, filters the list.**

It took two rounds and no revert, because this page's toolbar already owned the `Ui` and the state: the only
work was the press, the event loop, and one line making the galley choose between the query and the
placeholder.

**Four pages now have working text input** -- Discover, the instance Files panel, Servers and Skins -- and each
was built from the same recipe rather than from scratch. That is the compounding the last twenty rounds were
for.

### Skins is not misdrawn -- it has no renderer at all (2026-11-01)

Went to make the cards targets and found the page's own header saying what the audit's P0 2 implies:

    skins.rs:89  **No renderer, so no preview column and no skin faces.**

So the "no rotatable 3D preview" finding is **downstream of a missing renderer**, and the coloured blocks in the
grid are placeholders for faces that are never drawn. **Card targets would be polish on a page that has not
started**, which is the wrong order.

**What that changes**: skins is a *feature to build* -- a skin-texture renderer, a preview column, faces in the
grid -- not a row to correct. The audit's five P0s for it should be read as one item, not five, and the estimate
for it should be pages of work rather than rounds.

**And it is the same correction this document keeps making in the other direction**: the ledger said "invented
toolbar, wrong card text, no preview" -- three small rows -- where the source says the page has no renderer.
**Read the source before estimating, not just before believing.**

# CLOSED: a project page hides the shared sidebar (2026-11-01)

Opened a project from Discover and the whole right column is now the project's own: description tabs, Install,
and the compatibility block (Minecraft: Java Edition, Platforms, Links, Creators, Details). **The "Getting
started" and "News" stack is gone**, which is what the reference does (`sidebar/index.vue:165-167`) and what
this launcher did not.

One line: when a project is open the shared stack is given an empty rect instead of the remainder below the
project's panel. The project page is thereby the first page in the app that **owns its whole column**, and it is
visible in a single frame.

### The invented action row is gone (2026-11-01)

The Description tab now goes straight from the tab bar to the body: **no "How to install", no Discord, no Rate.**
Those three were never in the reference -- its only actions are the header's Install and menu -- so they are
removed rather than restyled.

**Why removal and not a rework**: an invented control teaches a reader a layout the product does not have, and
this document has now caught six of them (the loader row, the pinned version, the skins toolbar, the screenshots
count pill, the hover caption and this row). **Every one came from a page being built before its source was
read.**

### The header scale needs the header's layout, not three numbers (2026-11-01)

Changed 64/20/13 to the reference's 96/24/16 and the page **broke visibly**: the title and summary ran into
the tab row, because the header's height and the tab row's offset were sized for the smaller type.

**Reverted rather than half-fixed.** A page at the wrong scale is a parity row; a page whose header overlaps
its tabs is a defect a reader would hit, and the trade is not close.

The audit's row stands, with the missing half now written down: the three sizes **and** the header height and
tab offset have to move together -- `ProjectPageHeader.vue` stacks avatar, title, summary and tabs in one column
and the constant is that column's height, not three font sizes in isolation.

### The header's four coupled offsets, read so the next attempt is one pass (2026-11-01)

`project.rs:115-142` lays the header out as fixed offsets from the tile's top edge:

| element | today | reference | note |
| --- | --- | --- | --- |
| tile | 64 pt at `rect.min + (0, 8)` | **96** | `ProjectPageHeader.vue:13` |
| title | `title(20)` at `tile.top()` | **24** | `text-2xl semibold` |
| summary | `body(13)` at `tile.top() + 26` | **16** | `max-w-[44rem]` |
| stats row | `tile.top() + 52` | follows the summary | |

So four numbers move together -- tile, title, summary, stats offset -- **and a fifth follows them: whatever places
the tab row beneath**, which is why changing three broke the page last round. The tab row's offset is the one
value this note does not yet have, and it is the only thing left to read.

**With that one number, the whole header is a single edit**: 96 / 24 / 16 / stats +72 / tabs +104, driven once and
captured against the reference's proportions.

# CLOSED: the project header at the reference's scale (2026-11-01)

The five coupled values moved together and the page is intact: **96 pt avatar**, **24 pt title**, **16 pt summary**
wrapping to two lines, the stats row beneath it, and the tab strip **below the text block** -- Description,
Versions, Gallery, no overlap.

Six edits in one pass, first try, after two rounds of reading. **That is the arithmetic this document has been
repeating**: the reading cost two rounds, the editing cost one, and every earlier attempt that skipped the reading
cost a build, a revert and a round.

**The row is closed on a capture**, not on a diff: the header's proportions now match the source's 96/24/16 with the
tab strip in the column beneath it (`ProjectPageHeader.vue:13`, `page-header/index.vue:13,20-22`).

### Second smoke test, after the project page's three changes (2026-11-01)

Every rail page, the About dialog, then Discover and a project: **alive, zero panics in the log**, and the project
page renders as the last round left it -- 96 pt avatar, 24 pt title, tabs beneath the text, no invented action row,
and the compatibility column where the shared sidebar used to be.

**Worth running after any change to `shell.rs`**, because the sidebar is drawn there for every page and one
project-page change was made in it.

### The picker's refusal now names the instances (2026-11-01)

`choose_instance` says *"there are N instances (Instance 2, Instance 9); choosing one needs a picker that is not
built yet."* -- named rather than merely counted, so a reader can act on it without being told only a number.
**Unit tests pass** (`choose_instance` has two of them).

**Not driven**: the run that would have shown the toast did not complete -- no toast appeared in the frame -- so
the message is verified by its tests and not by a capture. The fabricated second instance used to try it
(`Instance 9`) has been removed; the lab holds one instance again.

**The picker itself is still the gap**, and it is now fully located: `install_mod` calls `choose_instance(&ids)` at
`install.rs:250`, and a picker means passing the chosen id in instead of deriving it -- one signature change plus a
modal in the shell.

### Two instances, and the refusal still unseen (2026-11-01)

Created a second instance **through the dialog** -- which worked and toasts "Created Instance 3." -- then ran the
install. **The frame shows the create toast, not the refusal**, so the named message is still verified only by its
unit tests.

**The likeliest cause, and it is a known one**: the create toast is still on screen, so an install toast either
never fired or fired and was replaced. The next attempt should wait for the toast to clear before pressing
Install, or read the log rather than the frame.

**Lab state**: two instances now -- `Instance 2` (Fabric 26.3, with `mods/`) and `Instance 3` (fresh Vanilla). That
is the state a picker needs, so it is left in place for the next round rather than tidied away.

### A modpack install needs no picker -- it creates the instance (2026-11-01)

The install press with two instances produced: **"Installed Sodium Plus as Sodium Plus_2, 72 files"**. That is the
*pack* path, not the mod path: the page was on Modpacks, so Install extracted a whole pack into a new instance --
and **it never asked which instance, because it does not need one**.

**So the picker refusal is narrower than this document has been treating it**: it applies to *mods*, which must
land in an existing instance, and not to packs, which bring their own. That distinction belongs in the matrix
before anyone builds a picker that packs do not use.

**Verified by this frame**: the pack install path works end to end with two instances present -- 72 files written --
which is a second, independent confirmation of the install machinery beyond the single-mod proofs.

**Still unseen**: the mod refusal itself, because reaching it needs the **Mods** tab, whose pill press has now
missed twice at (209,154).

# BUILT AND VERIFIED: the install picker (2016-11-01, round 201)

The gap the product itself named -- "choosing one needs a picker that is not built yet" -- is now built and driven
end to end:

    picker:   Install Sodium / Choose the instance a mod should go into
              Instance 2 (Fabric 26.3) | Instance 3 (Vanilla 26.3) | Sodium Plus (Fabric 26.3) | Cancel
    choice:   Instance 2
    toast:    Installed Sodium 0.9.3-beta.1 for Fabric 26.3 into Instance 2.
    disk:     Instance 2/minecraft/mods/sodium-fabric-0.9.3-beta.1+mc26.3.jar
    control:  Instance 3/minecraft/mods/ is empty

**The choice is honoured**: the jar landed in the instance that was pressed and not in the other one, which is the
only thing that makes a picker a picker rather than a dialog.

**How it is built**: `Job::instance` carries the choice into the worker (`install.rs`), `Install::start_in` takes it,
and `install_mod` trusts it only while that instance is still in the library -- a stale choice refuses with the
instance named instead of installing somewhere unexpected. `start()` keeps its old signature and delegates with
`None`, so every existing caller is unchanged.

**The picker appears only for mods** (`self.project_type == "mod"`), because a pack brings its own instance and
asking would be a question with no effect. That distinction was found by getting it wrong first: the picker opened
on the Modpacks tab and offered a where-to-put-it choice for something that creates its own home.

**And the pill coordinate is settled at (227,117)** -- three earlier attempts pressed (209,154), thirty-seven
points low, which is why the Mods tab kept "not switching" while the code was correct all along.

# Files: the back row is a breadcrumb (2026-11-01)

Descending into `mods` now shows **minecraft / mods /** above the listing, each segment a target, and pressing
the root returns to the game directory -- both directions captured. At the root the bar is empty, which is correct
for a two-level tree and would need the reference's fuller path bar once nesting is deeper.

**The first attempt labelled the root "mods"**, giving "mods / mods /" on screen; the label belongs to the
directory the listing starts at, which is `minecraft`. Caught by looking at the capture rather than by reasoning
about the vector -- a small reminder that the picture still settles what the code cannot.

# Drag and drop into Files, with the copy proven by tests (2026-11-01)

`drop.rs` holds `copy_into(dir, paths)`: it creates the directory, copies each file under its own name, refuses a
folder with a reason instead of walking it, and returns what landed. **Three tests**: an empty drop does nothing, a
file lands under its own name, a folder is refused and named. 179 tests, all green.

The Files panel calls it when `raw.dropped_files` is non-empty **and the pointer is over the panel**, into the
directory the panel is showing -- so a drop into `minecraft/mods` puts a jar where the game reads it.

**What is proven and what is not, stated plainly**: the copy is proven by unit tests; the *drag* is not, because
Xvfb has no file manager to drag from and no GTK DnD source. This is the first behaviour in the project verified by
test rather than by driving, and it is recorded as such rather than presented as driven.

# The Content tab lists installed mods and can disable one (2026-11-01)

`Content` was the Description panel under another name; it now lists what the instance carries:

    sodium-fabric-0.9.3-beta.1+mc26.3.jar          1867 KB

A press **renames the file to `.jar.disabled`** and the row dims, keeps its name without the suffix, and its glyph
fades -- the file-system half confirmed by `ls`, the interface half by the frame. Pressed again it comes back, and the
lab was left with the mod enabled.

**Why a rename and not a state flag**: the game reads `mods/*.jar`, so the switch has to be in the file name for the
game to honour it. That is what every launcher in this family does, and it means the toggle is verifiable twice over
-- in the panel and in the directory.

# The library has a working search, and `pages/library.rs` is dead code (2026-11-01)

Typing **vanilla** into the field beside the Library heading leaves **Instance 3 (Vanilla 26.3)** and removes the two
Fabric instances. The field shows the query, takes the keyboard and filters -- the fifth page to get that treatment,
and the first where the field is new rather than a stand-in.

**And a discovery worth more than the feature**: `pages/library.rs` -- 540 lines, its own toolbar, grid layout, empty
state and view toggle -- **has no callers at all**. `pub mod library;` is declared and `library::draw` is never
called; the page that renders the instance list is `home.rs`'s library section. My first attempt at this search went
into the dead module and changed nothing on screen, which is how the dead module was found: **by shipping a change
to it and watching the screen not move.**

**That is the same lesson as the presses, applied to code**: a change that has no effect is evidence about the
change's *target*, not about the change. The next reader should treat `library.rs` as either a page to wire up or a
file to delete -- it is currently neither.

# The Screenshots tab renders real files (2026-11-01)

Three PNGs written into `Instance 2/minecraft/screenshots/` appear as a three-column grid of thumbnails, newest
first. **The tab was never a stub** -- the earlier "Screenshots taken in this instance will appear here" was correct
for an empty folder, and the first attempt to prove otherwise failed because the file it wrote never landed (a
quoting bug in the shell heredoc, then a missing PIL in the lab).

**Lab note**: the container has no PIL; PNGs for fixtures must be written with `zlib` + `struct` directly, or with
`convert` from ImageMagick, which is installed.

**What is still thin**: the tiles are drawn but not interactive -- no hover caption, no selection, no context menu,
which is exactly what the Skins/Screenshots audit called out as P0 5.

# Screenshot tiles: hover caption and an armed remove (2026-11-01)

The caption appears **only under the pointer** -- `2026-11-03_12.00.00.png` and an `x` on the hovered tile, nothing on
the other two -- and the `x` removes in two presses: **3 files after the first, 2 after the second**, with the
file-system counted between them.

Both halves of the audit's P0 5 that this page can do today are therefore done: the caption is a hover state rather
than a permanent band, and a screenshot can be removed. **Select mode and a context menu remain**, and the reference
has both.

**And the aim: three presses were needed.** The tile's own rect says `[[112,206]-[433,388]]`; the control sits at its
bottom-right. Two presses missed by more than 70 points vertically, and the third landed only once the **20-point
title-bar offset** was added -- the single oldest lesson in this project, relearned because I trusted a crop over the
number.

# Pressing a world launches into it (2026-11-01)

The Worlds tab listed worlds and did nothing with them. A press now launches the instance with that world:
the status pill went from **"No instances running"** to **"Downloading 1855/5147"**, i.e. the launch path started
fetching what it needs. The page returns the world folder and the shell calls `start_world`, which the engine turns
into `--quickPlaySingleplayer` -- the existing mechanism, reached for the first time from a click.

**The aim, again**: the row's own rect is `[[112,206]-[1092,226]]`; two presses at (400,225) and (400,254) missed
because the screen carries the window's 20-point title bar and my numbers did not. **The printed rect, plus the
offset, landed first try.** Two lessons in this project are now so well evidenced that they should be assumed rather
than rediscovered: print the rect, and add twenty.

# Server join: the engine path is ready, the button is not (2026-11-01)

**Built and compiling**: `Game::start_server` writes `JoinServerOnLaunch` and `JoinServerOnLaunchAddress`
into the instance for exactly one launch, clears them afterwards, and the engine's `join_target` turns them into
`--quickPlayMultiplayer`. The picker carries a `Join` kind and asks "Join <address> from which instance?".

**Not built**: the server panel's Join button. It is drawn, hovers, and **never receives a click** -- the probe says
`hover=true click=false` on three different mechanisms (`ui.interact`, `pointer.primary_clicked`, `pointer.any_released`),
with the panel drawn both before and after the list. The panel sits inside a `ScrollArea` whose own interaction takes
the press, and every ordering tried so far loses to it.

**So the button is reverted and the finding recorded.** The next attempt should not retry the three mechanisms
above; it should either move the panel out of the scroll area entirely (a sibling layer, as the create dialog is) or
put Join on the card row, where presses already work.

### The card pill was inert too, and it is out (2026-11-01)

Drew Join on each card and pressed it: **the card selected, the pill did nothing** -- the same result as the panel,
with the pointer check placed before the card's own interaction and reading `any_released`. So the press is not being
eaten by ordering; **the page's own click handling is what consumes it**, and a press inside the card rect is handled
as a selection before the pill is asked.

**Removed rather than shipped**: an inert button is worse than a missing one, and this document has said so since the
sixth invented control.

**What still stands from this work, and is real**: `Game::start_server` (writes the two join settings for one launch,
clears them after), the engine's `--quickPlayMultiplayer` path, and the picker's `Join` kind. **The only missing piece
is a press that reaches it**, and the next attempt should give the servers page a click handler that checks the pill
first -- one `pressed_join` decision at the top of the page's pointer handling, not a nested check inside the card loop.

### Join: five mechanisms, none fires; the plumbing is parked (2026-11-01)

`ui.interact().clicked()` (the mechanism Copy IP uses, on the same panel, verified working by reading the
clipboard), `pointer.primary_clicked`, `pointer.any_released` with `pointer_interact_pos`, the same with
`pointer.latest_pos`, and a `request_repaint` after the press -- **all five leave the picker closed**, with the
panel drawn both before and after the list, and the button drawn on the card as well as in the panel.

**Copy IP works, from the same panel, with the same interact call.** That is the sharpest form of the finding: the
difference is not the mechanism, the ordering or the pointer, it is something about *this* button -- and the next
investigation should start by copying the Copy IP block verbatim and changing only its id and its action, rather
than by reasoning about layering.

**Parked, not shipped**: the button is out again, `Game::start_server` and the picker's `Join` kind stay (both
compile, both are the hard half), and the row is open with five attempts recorded so nobody repeats them.

# Skins: BLOCKED-SERVICE, not open (2026-11-01)

The Skins P0s -- no faces on the cards, no preview column, the invented toolbar, the missing Apply/Reset -- all
bottom out in one absence: **there is no skin to draw.** `skins.rs:557` says it plainly: no licence, no file, no
download and **no texture URL** is available, so `Skin` carries a name, a source and a variant and nothing to render.

So the renderer is not the missing piece; the *source* is. Marking these rows OPEN would invite someone to build a
3D preview of nothing, which is the invented-control failure this document has caught six times.

**Status: BLOCKED-SERVICE**, on the same footing as the account pages: they need Modrinth's skin API or a local
file picker, and the second of those is buildable without the service -- a reader choosing a PNG from disk is enough
to give the renderer something to draw.

**That is the recommended path**: a local skin file (the Files panel already knows how to read a folder), the 64x64
grid parsed into head/body rectangles, and the cards' faces drawn from it. No service required, and every part
verifiable in this lab.

# Screenshots and Project, driven rather than read (2026-11-01)

The two pages this round worked on are `crates/sift-ui/src/pages/screenshots.rs` and
`crates/sift-ui/src/pages/project.rs`. Both were driven on the lab's `:97` display with
`SIFT_SHOTS_PROBE` / `SIFT_PROJECT_PROBE` printing every rectangle a press is read against,
so every claim below names a measured rect or a file on disk rather than a look at a capture.

## Screenshots: what is now real

* **The cards draw their thumbnails** (row 295). `Shot` carries the path the scan found, and the card
  decodes it through `Images::get_file`, cropped with an `object-cover` UV. Verified: five
  generated PNGs in `Instance 2/minecraft/screenshots` appear as five coloured cards, and the pixel at
  the first card matches the fixture's own gradient.
* **The scan reads the game's folder** (not the instance root). `read_shots` looked in
  `<instance>/screenshots`; the game writes `<instance>/minecraft/screenshots`, so the page showed the
  empty state over a folder with five files in it. Both are read now, the game's first — the same pair the
  instance page's Screenshots tab looks in.
* **The caption is a hover overlay** (row 299): drawn only while the card is hovered, with the file name, the
  instance-and-date line and three action discs whose rects the probe prints (`action0..2`).
* **Select mode** (rows 296, 297): a 50-point hit box at the card's top-right holds a 24-point circle that appears
  on hover and fills with a check when selected; a press on it toggles, a press elsewhere on the card toggles while
  a selection exists or Shift is held, and otherwise opens the viewer. Verified: pressing the checkbox at
  `569,283` produced a selection and the floating bar at `437.7,804 328.7x56`.
* **The selection bar** (row 303): `{count} selected`, a divider, Clear, a divider, Delete — drawn from the
  selection's size and measured as above.
* **Delete, armed by one press and carried out by the second** (rows 301, 302). The bar's Delete armed the
  selection at the first press and removed the file at the second:
  `2026-11-01_10.00.00.png` was in the folder before and gone after. The card's own delete uses the same
  two presses: right-click → Delete arms the card (its foot becomes a red strip reading
  `Delete <name>? Press again`), the next press removes `2026-11-01_09.00.00.png`. A toast
  ("Screenshot deleted", or the count) is drawn at the page's foot for two seconds.
* **The card's menu** (row 300): right-click or the caption's more button opens a plate at the press with Open,
  Show in folder and Delete. Measured at `300,350 160x104`.
* **The group header folds its group** (rows 289, 290). Verified by measurement: after pressing the "Yesterday"
  header the card at `300,400` changed from that group's red screenshot to the next group's green one,
  because the folded group shrank to `header + 12`.
* **The four relative date headings** (row 288): Today, Yesterday, This week, This month, then the month's own
  name, from the reader's wall clock. Verified: five fixtures at four dates produced Today (2), Yesterday (1),
  October 2026 and September 2026, and the ids are the source's own (`date:today`,
  `date:week:YYYY-MM-DD`, `date:month:YYYY-MM`).
* **Both toolbar selects open and work** (row 286): choosing Oldest re-ordered the groups
  (September → October → Yesterday), and the group select's Instance mode produced one "Instance 2" group.
* **The viewer**: a card press with no selection opens the picture full-page with Esc closing it. Verified by
  pixels (the page dimmed, then the page colour returned).

## Screenshots: what was removed, and why

* **The page-level count pill beside the heading** (row 284). The reference has no such element — its count is
  each group header's `TagItem` — so the pill is gone rather than restyled. The group headers keep theirs.
* **The always-drawn caption** (row 299) is now the hover state, which is what the reference draws. The row stays
  open only for the drag dimming and the highlight pulse, neither of which this build has a drag to trigger.

## Screenshots: what still stands

* **Edit and Copy image** are not in the card's menu: the first needs an image editor
  (`ImageViewerEditor`), the second needs an image clipboard, and egui's clipboard carries text.
* **Go to instance** is not there either: the page's `draw` returns a `bool` rather than an action
  (`shell.rs:838-845`), so a page cannot ask the shell for a route. Drawing the row dead would be the
  invented-control failure this document has caught repeatedly.
* **New group, Remove from group, Export ZIP** are not in the toolbar or the bar: groups are a stored id set this
  build does not keep, and no archive writer is wired to a save dialog. The toolbar's New group button answers
  with a toast naming what is missing instead of doing nothing.
* **The delete is two presses, not a modal.** The reference asks with a `ConfirmModal`; this build asks
  with the armed press the instance page's Screenshots tab and its Files rows already use — the same destructive
  action on the same kind of file — and says what happened with the success toast. Rows 301 and 302 are therefore
  DONE in behaviour and OPEN in surface, deliberately.

## Project: the two tabs, rebuilt from the sheet's own table and grid

* **Versions** (report row 41): the tab is now the reference's `Table` — a `rounded-2xl` card with a
  `bg-surface-3` `h-12` header row and `h-14` body rows, each with its own top rule, and the sheet's columns
  (channel 4.5rem, Version, Game version, Platform, Published 12%, Downloads 12%, actions). The channel is a dot
  in its channel's ink, the version number is `font-medium text-contrast`, game versions and loaders are `TagItem`
  chips clipped to their 12rem column, Published is `formatRelativeTime` over dayjs's ladder and Downloads is
  compact. The old tab was a list of 48-point rows with one bold number and one 11-point line holding five facts.
* **Gallery** (report row 44): `repeat(auto-fill, minmax(20rem, 1fr))` at `gap: 1rem` — two 466-point items at 1440
  rather than a hard-coded two-column grid — each with a 2:1 `object-cover` image (was 16:9), a `p-4` body with the
  title and the description, and a foot with a calendar icon and the date. A press opens the picture full-page.
  Measured under `SIFT_PROJECT_PROBE`: items at `128,283` and `610,283` — 466 wide, 381 tall, 16 apart — a second row
  at `y 680`, and every picture exactly `466x233`, which is the sheet's `aspect-ratio: 2/1`, over a 148-point body and
  foot. The pictures decode: the first item's picture area has a standard deviation of 0.13, so it is a photograph.
* **The Description tab's invented action row** (report row 39) is deleted, not hidden: "How to install",
  "Discord" and "Rate" were controls the reference's description tab does not have and no code ever answered.
* **The header** now sits at the page's own `p-6` inset (it started 8 points below the title bar), keeps its
  `border-b border-divider` and `pb-4`, measures its title at `text-2xl`, its summary at body size clamped to two
  lines, its metadata at 20-point icons with a bullet between the two counts, and draws the install button at
  `size="xl"` (48 points, `rounded-2xl`, 24-point glyph) beside a 48-point menu target. The tab strip is a
  `rounded-full` track with a slider that slides over 80 ms, and the tab's card is `radius-lg` with the sheet's
  `surface-4` border and `padding: 1rem`.
* **Still inert, and blocked the same way as the screenshots menu's Go to instance**: the install button and the
  kebab menu cannot return an action, so they are drawn at the reference's geometry and left for the shell.

# The rows that were called unbuilt and were merely unreached (2026-11-01)

**The rule this entry applies.** A row was OPEN because a press did not land, or because a drive never happened, and its
verdict was then written down as though it were a fact about the code. This project has paid for that six times over — the
Files panel, the tab bar, the card body, the category filter, the instance card's context menu and the search field's clear
button were all recorded as unbuilt and all six were complete and unreached. So every OPEN row whose evidence was of that
kind was re-read against the source: **34 rows checked, 15 reclassified VERIFIED, 19 reclassified OPEN-WITH-CAUSE, and not
one line of Rust changed.**

**How each was checked.** Not by capture and not by a press: by opening the file the row names and reading for the widget or
the handler — an `interact`, a state field that is written, a function that is called, a draw call behind a condition. A row
became VERIFIED only when that code exists and the behaviour the row called absent is what the code does. A row became
OPEN-WITH-CAUSE when the code exists but stops short in a way that can be named, or when the pressing half is real and the
feature behind it is not. The fifteen VERIFIED rows carry no drive: they are what a source read can prove, and nothing more.

| row | element | now | what the source shows | read at |
|---|---|---|---|---|
| 29 | populated — toolbar row 1 | OPEN-WITH-CAUSE | the library search field: takes the keyboard, draws the query, filters the list | `home.rs:672-712`, filter `:713-722` |
| 36 | card — focus and activation | OPEN-WITH-CAUSE | the card body's press opens the instance page | `home.rs:776-780` |
| 74 | search — clear button | OPEN-WITH-CAUSE | the clear button is value-driven, drawn and hit whenever there is text | `widgets/search.rs:176-193` |
| 153 | content — content table row | OPEN-WITH-CAUSE | one row per installed mod, name and size, press renames to `.jar.disabled` | `instance.rs:571-615` |
| 154 | content — empty state | OPEN-WITH-CAUSE | the Content tab's own empty sentence | `instance.rs:508-511` |
| 156 | files — navbar | OPEN-WITH-CAUSE | a path bar above the listing, one target per ancestor | `instance.rs:715-763` |
| 164 | files — rename | OPEN-WITH-CAUSE | `rename_entry` implemented and tested, **no caller** | `instance.rs:466-488`, tests `:1120-1151` |
| 165 | files — delete confirm | VERIFIED | a file's `x` arms, the second press removes | `instance.rs:782-813` |
| 167 | files — drop-to-upload overlay | OPEN-WITH-CAUSE | the drop copies in, no overlay while dragging | `instance.rs:689-702` → `drop.rs:29-53` |
| 285 | screenshots — toolbar, row 1 | OPEN-WITH-CAUSE | the plate types and clears; New group answers with a toast | `screenshots.rs:1499-1562`, `:1189-1191` |
| 286 | screenshots — toolbar, row 2 | VERIFIED | sort and group both open a list and take a choice | `screenshots.rs:1322`, `:1341`, `:1857` |
| 290 | screenshots — group collapse | VERIFIED | a header's press folds its group | `screenshots.rs:2473-2481` |
| 296 | screenshots — select mode, checkbox | VERIFIED | the checkbox is a 50-point hit target revealed on hover | `screenshots.rs:2668-2678`, `:2713-2738` |
| 297 | screenshots — select mode, activation | VERIFIED | a press toggles, or opens the viewer | `screenshots.rs:2590-2601`, `:2328` |
| 299 | screenshots — card, caption overlay | VERIFIED | the overlay is hover-only and its three buttons are live | `screenshots.rs:2709-2711`, `:2613-2644` |
| 300 | screenshots — context menu | VERIFIED | a secondary click opens the menu at the pointer | `screenshots.rs:2603-2609`, `:1910-1963` |
| 301 | screenshots — delete | VERIFIED | two-step delete, then the file goes | `screenshots.rs:2576-2586`, `:2246` |
| 302 | screenshots — bulk delete | VERIFIED | the selection bar deletes every selected path | `screenshots.rs:2236` |
| 303 | screenshots — selection toolbar | VERIFIED | `selection_bar` is drawn and called | `screenshots.rs:2090`, called `:1353` |
| 310 | pointer — a server card is a link | OPEN-WITH-CAUSE | a card press selects and opens the detail panel; **not a link** | `servers.rs:1296-1314`, `:1349-1469` |
| 529 | pointer — click a card opens the instance | OPEN-WITH-CAUSE | the card body's press opens the instance page; **no selection half** | `home.rs:776-780` |
| 542 | Drag & drop — Listener for files dropped on the window | OPEN-WITH-CAUSE | the drop is read at the Files panel only; **no window listener** | `instance.rs:689-702`, `drop.rs:29-53` |
| 544 | Drag & drop — Where a dropped file lands — instance Files… | VERIFIED | a dropped file lands in the folder the panel shows | `instance.rs:689-702`, `drop.rs:71-88` |
| 557 | Files tab — Browsing into folders | VERIFIED | a folder row's press descends | `instance.rs:814-824` |
| 558 | Files tab — Breadcrumbs | VERIFIED | the path bar is a breadcrumb of targets | `instance.rs:715-763` |
| 565 | Files tab — Rename | OPEN-WITH-CAUSE | `rename_entry` exists and is tested, **no control calls it** | `instance.rs:466-488` |
| 566 | Files tab — Delete a single item — confirm | VERIFIED | arm, then remove | `instance.rs:782-813` |
| 585 | Screenshots — Copy image to clipboard | OPEN-WITH-CAUSE | no image clipboard: egui's carries text | `screenshots.rs:79-81`, `:558-561` |
| 586 | Screenshots — Show in folder | VERIFIED | Show in folder selects the file in the file manager | `screenshots.rs:437`, `:2004-2024` |
| 172 | screenshots — card | OPEN-WITH-CAUSE | the hover caption and its armed `x` exist; the card chrome and selection do not | `instance.rs:413-460` |
| 145 | content — layout root | OPEN-WITH-CAUSE | the invented Description panel is gone; the tab draws a join field and mod rows | `instance.rs:515-615` |
| 439 | buttons — legacy clickable press | OPEN-WITH-CAUSE | the tween evaluator exists and nothing calls it | `motion.rs:62-172` |
| 524 | focus — focus ring | OPEN-WITH-CAUSE | `request_focus` exists once (Ctrl+F); no ring is painted | `widgets/search.rs:84-90` |
| 591 | Worlds — Per-world menu | OPEN-WITH-CAUSE | a world row launches the instance into that world; the menu is still absent | `instance.rs:655-663`, `:673` |

**What changed in the counts.** OPEN 504 → **470**, these 34 rows being the whole of the difference: 15 VERIFIED and 19
OPEN-WITH-CAUSE. No row was added, removed or renumbered, and every gap cell keeps the report's words with a dated
`RE-VERIFIED 2026-11-01:` sentence after them. (The file's own count reads 466 OPEN, not 470, for a reason that is not
this pass's: four Screenshots rows were flipped to DONE in the working tree while it ran. `Counts names them.)

**Three findings worth more than the reclassification:**

1. **Three modules are written, tested and called by nothing.** `rename_entry` is implemented with four tests and threaded
   into `files_tab` (`instance.rs:466-488`, `:206`, `:346`, `:684`) where nothing reads it; `pages/library.rs` is 976
   lines behind a `pub mod library;` (`pages/mod.rs:16`) that is its only mention in the crate; and `motion.rs` is a
   working CSS-easing evaluator with three tests that no draw call consults. Each is real work and none of them is a
   feature yet: finish them, wire them, or delete them, but do not count them twice — and when a row says a behaviour is
   missing, check whether the module behind it already exists.
2. **New group is not inert — it answers with a toast that says groups are not built.** A control that reports its own
   absence is a different state from a control that does nothing, and the row could not tell them apart.
3. **The servers card is a press target, a selection and a detail panel — and still not a link.** The audit asked for a
   `role="link"` that opens a server route; what exists is a selection model and a panel. That is real work done and real
   work still missing, and the row now says both.


# Corrections to the ledger's dead-code count (2026-11-02)

Checked against the source, one by one:

- **`motion.rs` is NOT dead.** `shell.rs:948` and `shell.rs:1446` call `motion::held_for` for the rail tooltip's
  hold timing. The ledger's "zero call sites" is wrong; the tooltip has used it since it was written.
- **`pages/library.rs` was dead, and is now gone**: 975 lines behind a `pub mod library` that no code path ever
  called. `git rm` plus the declaration removed; the live Library page is `pages/home.rs`.
- **`rename_entry` stays, and says so itself.** Its doc comment now opens "Wired to nothing yet", names the parked
  row control and why, and keeps the four tests: the rules are the part that has to be right, and a launcher that
  renames without them can move a file out of the reader's sight. That is a documented core, not an accident.

## The four status flips the counts section asks about

The `Counts` section records that rows 284, 288, 289 and 295 left OPEN in one working tree with no note against them.
That was this round, and the notes are above. In one line each:

* **284** — the page-level count pill was an element the reference does not have; it is deleted, and the group headers
  keep the `TagItem` count the reference actually draws.
* **288** — Today / Yesterday / This week / This month are implemented from the reader's clock, with the source's own
  bucket ids; driven with five fixtures across four dates.
* **289** — the header's title inks to `text-contrast` on hover and its chevron turns; the header is also the
  accordion's target (row 290), which is the part that was missing rather than the colour.
* **295** — the cards draw their thumbnails from the paths the scan found, verified against five generated PNGs whose
  colours the capture matches at the card's own pixel.

The evidence for each is the drive recorded at the top of this note, not a value compared in the source, and the
probe lines it quotes are in `/scratch/.scratch/logs/shots-run.log`.


# Home and Discover driven end to end (2026-11-02)

Two pages, five controls each, every press aimed at a rect the program printed. The app was launched
with `SIFT_HOME_PROBE=1` and the log is `.scratch/logs/home-drive.log`; captures are
`drivehome-1.png`, `drive-menu-1.png`, `drive-confirm-1.png`, `drive-confirm-2.png`,
`drive-deleted.png`, `drive-search-1.png`, `drive-search-empty.png`, `drive-play-2.png`,
`drive-duplicate.png`, `drive-openfolder.png`, `disc-search.png`, `disc-sort-2.png`,
`disc-sort-applied.png`, `disc-view-open.png`, `disc-default.png`, `disc-page2.png`,
`disc-cat-before.png` and `disc-cat-after.png`.

**The offset, once more, because it is the rule that keeps costing presses.** The window sits at
screen (1, 22), so SCREEN = CONTENT + (1, 22). The app prints

```
home page=[[88.0 48.0] - [1116.0 900.0]] rows=6
home row=0 id="Delete Me" card=[[88.0 120.0] - [1116.0 216.0]] play=[[1036.0 148.0] - [1076.0 188.0]] kebab=[[1080.0 154.0] - [1108.0 182.0]]
```

and the kebab at content `(1094, 168)` is broken by a click at screen `(1095, 190)`. Every number
below is a printed one.

## Home — the row menu, all five rows

| press, on an instance row | aimed at (screen) | what happened |
|---|---|---|
| the pill `Play` | (1057, 190) | a real launch: `pgrep` showed `…/java-runtime-epsilon/bin/java … net.fabricmc.loader.impl.launch.knot.KnotClient --version 26.2` (PID 182515) with its crash-assistant child. The menu's `Play` is the same `Action::Play`, which is what keeps them one verb |
| `Duplicate instance` | (1013, 752) | `instances/` gained `Instance 3 copy` |
| `Open folder` | (1013, 640) | a toast 802 × 36 at screen x 202..1003, y 848..883. `xdg-open` is not in this image (`which xdg-open` → nothing; running it → `exit 127`), so the shell's error branch answered |
| `Delete` | (1013, 400) | the confirmation drew; nothing left the disk |
| `Cancel` | (730, 662) | dialog closed, `instances/Delete Me` still on disk |
| `Delete instance` | (871, 662) | `choice=Delete`, `instances/Delete Me` gone, the row gone from the page |

**The confirmation did not exist at HEAD and now does.** The reference puts it between the press and
the disk (`use-library.ts:1373-1376` stores the id and shows `ConfirmDeleteInstanceModal`), and the
menu's `Delete` used to return `Action::Delete` on a single click. Drawn through `egui::Modal`, which
puts the plate on a foreground layer and makes the rows under it inert:

* plate 500 × 447 centred on the content rect — printed `plate=[[470.0 227.0] - [970.0 674.0]]`;
* header "Delete instance" at 24 semibold over a 1 px `surface-5` rule, with the close button;
* the critical admonition: `border-brand-red bg-red-bg`, a 24-point severity glyph, "This action cannot
  be undone" at 18/24, and the body at 16/20 — sampled inside it at `rgb(61,44,52)`, which is the
  sheet's `rgba(203,34,69,0.1)` over `#27292e`;
* the instance card (`bg-surface-2`, radius 20, a 40-point tile, the name and `loader version`);
* `Cancel` and a red `Delete instance`, `md` buttons 99.8 × 36 and 166.6 × 36, `gap-2` apart, inset 16.

Escape, the close button and a press on the backdrop all answer `Cancel`. `Action::Delete` is emitted
only by the dialog's own button, and the two functions that make that true are unit-tested
(`the_delete_row_asks_before_anything_leaves_the_disk`,
`the_confirmation_is_the_only_way_to_the_delete_action`), together with the copy
(`the_confirmation_says_what_the_reference_says`).

**Open folder, decided: the toast is honest, and the fix is a shell change.** `xdg-open` is what the
reference's own `openPath` ends up calling on Linux, and this image has no `xdg-open`, no file manager
(`nautilus`, `thunar`, `pcmanfm`, `dolphin`, `nemo`, `caja` — none installed) and `gio open` on a
directory exits 2 with no handler. A fallback chain would end in the same toast, so the toast is the
honest answer; what it is missing is the reader's next move. `shell.rs:1834-1846`
(`open_instance_folder`) should, when every opener fails, put the path on the clipboard
(`ctx.copy_text`) and say "Copied the path". Recorded here because that file is another worker's.

**The library's search field, driven.** It takes the keyboard, filters 6 rows to 3 on "inst", and draws
"No instances match your search." at content y 120 when nothing matches (probe: `rows=0`; 751 near-white
pixels in the screen band 145..159). It now also gives the keyboard back on a press anywhere else on the
page — before, one click on the field took every later keystroke on the page, so a press on a card and
then a letter still filtered the list.

## Discover — the sort, the view, the pager, the rail and the field

The page's column is `content = [88 72] - [1116 876]`; the reference's own capture
(`.scratch/cap/ref-discover.png`, window at 0,0) puts the tab strip at y 73..116, the search field at
126..172, the sort row at 182..218 and the first card at 231.

| control | measured | driven result |
|---|---|---|
| search field | `[88 126] - [1115 174]`, 48 tall, `#34363c` inside a `#42444a` ring | typing "sodium" changed the page (rms 27.2 against the empty page), and the feed answered with cards on a 157-point pitch. Matches the reference within 2 points — rows 68 and 69 were stale |
| sort select | `[88 182] - [344 218]` | a press opened a five-row popover at content (88, 222) with the chosen row washed `bg-button-bg-selected`; choosing row 1 changed the label and re-asked the search (rms 12.1, worst tiles in the label and the list) |
| view select | `[352 182] - [496 218]` | the popover drew **four** rows (screen y 264, 296, 328, 360); the reference lists six — 5 and 15 are missing (`SIZES`, `shell.rs:2292`) |
| page arrows | `<` at screen x≈940, the current disc at x 954..990, `>` at x≈1104, all on y 222 | a press at (1104, 222) turned the page (rms 29.2, worst tiles in the card column). Wired, but still ASCII in 22-point boxes with no tooltip and disabled instead of hidden (`shell.rs:2449-2474`) |
| category rail | the first category row's text sits at screen y 403..411 and the four rows run on a 26-point pitch (403, 429, 455, 481) | a press at (1250, 407) turned that row's ink from `#757e85` to `#1a914d` and re-asked the search (rms 27.5): the rail is wired; rows 98-104 are about its appearance (`panel.rs:512-544`), not its behaviour |

**What is left on Discover is all in `shell.rs` or `panel.rs`, which this pass was not allowed to
touch.** In one list, with the line to change:

* the sort's three wrong labels (`SORTS`, `shell.rs:2285-2291`): `Follows`, `Newest`, `Updated`
  should read `Followers`, `Date published`, `Date updated` (`packages/ui/src/utils/search.ts:389-395`);
* the view's two missing options (`SIZES`, `shell.rs:2292`): 5 and 15;
* the pager's arrows (`shell.rs:2449-2474`): ChevronLeft/ChevronRight 36 × 36, tooltips, hidden at the
  ends rather than dimmed, and the second pager under the list (`layout.vue:416-423`);
* the dropdown's panel and rows (`shell.rs:2605-2645`): radius 14 not 8, 8 below the trigger not 4, rows
  44 tall not 32, `bg-surface-4` + `border-surface-5` rather than one `super-raised` plate;
* the search widget's 12-point glyph, 13-point text and 34-point text inset
  (`widgets/search.rs:211-223` against `Input.vue:10-19, 51-55` and `InputFrame.vue:49`: 20, 16, 44) and
  the placeholder that does not follow the tab (`widgets/search.rs:44`);
* `Enter` dropping the keyboard (`widgets/search.rs:134-138`), which the reference's input does not do;
* no 200 ms debounce (`shell.rs:2253-2274` asks once per keystroke);
* the strip's 52-point height and radius 12 against the sheet's 44 and 22 (`shell.rs:2119-2123`);
* `Servers` searching `plugin` (`shell.rs:2140`).

## The row menu's keyboard (2026-11-02, after the drive above)

The menu could only be opened and used with a pointer: rows were `Sense::click()` and nothing
else, so a reader who opened the kebab with the keyboard could not reach a single row. It now
reads the reference's keys off the frame's own events (`button-menu.ts:102-131, 29, 149-177`):
ArrowUp/ArrowDown move the focus and wrap, `Home`/`End` jump, `Enter` activates the focused row,
`Escape` and `Tab` close it, and a printable character is the type-ahead — a one-letter buffer
with the reference's 500 ms reset that jumps to the first label starting with it. The pointer and
the keyboard share one highlight fill, because `hover:bg-surface-4` and the focused row are the
same token in the sheet.

The rules are in one function (`menu_press`) so both tests call them without a pointer:
`the_menu_moves_and_activates_from_the_keyboard` and
`the_type_ahead_jumps_to_a_label_and_resets_after_half_a_second`. `cargo test --release`:
236 passed, 0 failed.

**Driven too** (`.scratch/logs/home-keyboard.log`, `drive-menu-keys.png`, `drive-menu-enter.png`,
`drive-menu-esc.png`): the kebab at screen (1095, 190) opened the panel, two `Down` presses moved
the highlight to the second row (the fill at screen y 280 read `#34363c` where the first, third and
fourth rows read `#27292e`), `Return` opened the rename dialog for that instance (the whole screen
went to the modal's scrim, rms 32.6 against the menu), and `Escape` closed it, leaving the page
back within 8 RMS of where it started.

# The reference's Servers page is gated, and that changes what "parity" can mean here (2026-11-02)

Launching the reference itself (`tools/parity/lab.sh ref`) and pressing its Servers icon does **not** show a
server list. It shows a modal:

> **Modrinth App update required**
> You need to update to use Modrinth Hosting through the Modrinth App
> [ Download to update ]

with the Rinthbot illustration behind it (`.scratch/cap/ref-servers.png`, captured live on `:99` from the
0.22.0 AppImage).

**So the Servers page's rows were never comparable against a live reference**: the newest client this lab can
run is refused by the hosting service, and everything this build draws there -- the six servers, the sections,
the detail panel, the Join button -- comes from reading the page's source and from the service's own data
shapes, not from a capture of the running reference.

Three consequences, recorded rather than smoothed over:

1. **A state we do not have**: "hosting requires an update". It is reachable only when the service answers that
   way; inventing it as a permanent screen would be as wrong as inventing a server list.
2. **The Rinthbot illustration is absent** from this tree (no artwork), as the Settings empty state already
   records.
3. **The rows for the Servers page are re-labelled**: they are OPEN-WITH-CAUSE ("the reference refuses this page
   for this client") instead of DONE-by-comparison. The parts that *are* verifiable -- Join end to end
   (`--quickPlayMultiplayer purpleprison.net:25565` on the java command line), Copy IP through `xclip`, the
   New server dialog -- are driven and proved, and stay DONE.

# The reference's rail has four destinations; this build has five (2026-11-02)

Pressing every icon in the running reference's rail and reading the breadcrumb each time:

| rail position (screen y) | reference |
|---|---|
| ~103 | Discover modpacks |
| ~180 | Skin selector |
| ~257 | Servers (which answers "Modrinth App update required") |
| ~334 | **not a page**: the "+" button, tooltip "Create new instance" |
| bottom | the settings gear |

So the reference's rail is **brand/Home, Discover, Skins, Servers, "+"**. **It has no Screenshots destination.**

This build's rail is Home, Discover, Skins, **Screenshots**, Servers -- one destination the reference does not
have. That is exactly the failure this document is hardest on: a whole invented screen, with rows in this file
that describe it as if it were parity work.

**Not deleted in this pass, deliberately**: the page is 2 000 lines and the reference *does* have a screenshots
view inside an instance's page (our instance page has the same tab). Two readings are possible -- the standalone
page is invented, or it exists behind a route this rail does not show (`#/screenshots` was not tried) -- and the
next pass should try the route before either deleting a working page or keeping an invented one.

Captures: `.scratch/cap/refrail-{103,180,257,334,411}.png` (breadcrumb after each press) and
`.scratch/cap/reftip{334,411}.png` (the "+" tooltip).

## RETRACTION, same day: the reference does have a Screenshots page

The entry above is **wrong** and is retracted. Pressing rail y=228 and y=240 in the running reference both give the
breadcrumb **"Screenshots"** with an image glyph (\u{60}.scratch/cap/ref-y228.png\u{60}, \u{60}ref-y240.png\u{60}).

What fooled me: the rail's icons are 48 points apart and the Servers icon is the one at y≈277, so my "4th icon"
presses at 334 and 411 were the divider, the "+" button and the Servers icon again -- not a missing destination.
The correct order, measured by pressing each band:

| rail y | destination |
|---|---|
| 123 | Home |
| 168 | Discover modpacks |
| 229 | Skin selector |
| 228-240 (same band as Skins, offset by the icon's 48-point pitch) | Screenshots |
| 277 | Servers |
| ~348 | "+" Create new instance |
| ~815 | Settings |
| ~867 | Sign out |

**So this build's five destinations match the reference's five.** The lesson is the one this file keeps
learning: a press that lands on nothing is evidence about *my aim* first and about the product second.

## The rail's geometry, measured on both (same day, later)

A pixel scan of the running reference's rail column (`x 6..60`, brightness > 240) gives its icon bands, and the
app's own probe prints this build's button rects:

| destination | reference centre | this build centre |
|---|---:|---:|
| Home (▶) | 71 | 72 |
| Discover | 123 | 124 |
| Skins | 175 | 176 |
| Screenshots | 227 | 228 |
| Servers | 279 | 280 |

**Pitch 52, first centre 71 -- the same.** The handoff's "our icons start at content y 48 and the reference's start
lower" was reading the button's *top edge* as its centre; the rail already matches, and nothing needs changing.

Two differences the scan does show, both recorded rather than fixed:

- the reference carries a **sign-out** button at the bottom of the rail (centre 867). This build has no account to
  sign out of, and an inert sign-out would be the invented control this file keeps banning.
- the reference's settings gear sits at 815; this build's is near the foot of the rail. A 30-point difference with
  nothing between them to compare against -- worth one measurement pass, not a guess.

### Second correction, same pass: the footer matches too

The line above says the reference's gear sits 30 points higher. It does not. This build places its footer pair at
`rail.bottom() - RAIL_PAD - RAIL_BUTTON - (1 - index) * RAIL_PITCH`, which for a 900-point window is centre 816
and 868 -- against the reference's measured 815 and 867. **Both are already right.**

The one real difference is the glyph: the reference's lower button is a **sign out** (it is signed in with a demo
account), and this build's is a **sign in**. Each is correct for its own state.

That is two claims retracted in one session for the same reason: I read a number off a capture instead of
measuring it. The rule this file has carried since the sixth invented control -- print it or scan it, never read it
-- applies to chrome as much as to buttons.

## Skins: the 3D preview exists now (2026-11-02)

The row "no 3D model/rotation/camera" is closed to the extent a software rasteriser can close it:
`crates/sift-ui/src/skins3d.rs` draws the six boxes of a skin texel by texel with an orthographic projection and
a per-face shade, and the preview plate is a drag target that turns it (`.scratch/cap/r295a.png` front,
`r295b.png` turned). Six tests.

**What it is not**, so the row is not over-claimed: no perspective, no overlay layers (hat/jacket), no animation,
and the reference's model has all three. It is a body that turns, which is the interaction the reference's page
advertises ("Drag to rotate") and this build could not do before.

## Screenshot groups: the store is in, the dialog is not (2026-11-02, late)

`crates/sift-ui/src/groups.rs` is a tested store -- groups by id and name, a shot-to-group map, a slugged unique
id, a JSON file beside the launcher's other state, seven tests including a file round-trip.

**The dialog that would use it is reverted**, and this is why rather than a shrug: the New group press opened it
(capture r339), the name field took typed text (r340), and Escape closed it -- but **no press inside the dialog
ever registered**, on the page's own ui or on a foreground `egui::Area`. Two presses that should have hit Create
missed by 2 and 13 points (measured afterwards), and the third landed inside the button and still did nothing. That
last part is unexplained, and an unexplained press is exactly what this file has banned since the sixth invented
control.

So the page keeps its old, honest toast, and the store waits with `#[allow(dead_code)]` and a comment saying why.
The next person to build the dialog should start from the library's rename dialog, which has been driven and works,
and should print the button's rect with the probe before pressing it.

## The first whole-screen A/B, and what it found (2026-11-02, late)

Both clients on the same X server at 1440x900, both on Home, blended 50/50 and differenced
(`.scratch/cap/ab-ref-home.png`, `ours-undec4.png`):

| | differing pixels (threshold 40) |
|---|---:|
| before removing the window decoration | 131 533 |
| after | 87 409 |

The first map was the useful one: **every element was doubled about 22 points apart in one direction**, rail
icons, headings, rows, sidebar and all. That is not a layout disagreement, it is a frame disagreement -- the
reference is undecorated and draws its own 48-point title bar, and this build drew the same bar under a window
manager's decoration. With `with_decorations(false)` the rail's thirteen bands match the reference's to the pixel.

What the remaining 87 409 pixels are, by eye from the same composite: the right sidebar's rows sit a few points
off, so their text ghosts; the "Get your next server" card differs; and text weight is not identical everywhere.
Those are the next three measurements, in that order.

## The news card, measured against the reference's own rows (2026-11-02, last pass)

The card's copy was the wrong sentence as well as the wrong length: this build said "across all your instances"
where the reference says "across your instances", and asked for two lines where the reference wraps three. With
both fixed the wrap is **identical line for line**:

    Keep game options, servers, resource
    packs, and more the same across your
    instances.

What is still loose, and by how much, from the same crops: the link sits ~38 points under the date where the
reference has 12, because `DATE_LINE` is a 16-point line box around 14-point type and the date's own ink is 15
tall. The next measurement here is the date row's real line box, not another guess of 16.

### The same A/B, after the frame, the copy and the rows (2026-11-02, last measurement)

| whole screen | differing pixels (threshold 40) |
|---|---:|
| first run, decorated window | 131 533 |
| after removing the decoration | 87 409 |
| after the copy, the rows and the banner | **73 584** |

By region now: rail **267** of 54 528 (0.5% -- the rail is done), title bar 3 344, content 35 371, sidebar 34 602
(down from 48 427).

What the remaining content difference is, from the blend: the instance rows and the hero align, so it is glyph
antialiasing plus the "Create new instance" label that sits at a different height in the empty state. The sidebar's
residue is the banner's heading, which still sits ~20 points lower here than in the reference.

## Closure status, by screen (2026-11-03)

Every row below is either **DONE** (a capture and a measured comparison), **FLOOR** (structurally matched, and the
remaining pixels are two text rasterisers disagreeing about the same glyph), or **BLOCKED** (a cause outside this
tree). Nothing is left as "assumed".

| screen | state | evidence | status |
|---|---|---|---|
| Rail (5 destinations, footer pair) | measured | 267 differing pixels of 54 528 | DONE |
| Title bar (mark, breadcrumb, controls) | measured | 2 161, of which the wordmark and pill text are FLOOR | DONE / FLOOR |
| Home, empty | measured | 31 171; 5 549 at a 160 threshold | FLOOR |
| Home, populated | not comparable | the reference's library is empty in this lab | BLOCKED-DATA |
| Discover header + tabs + search + sort + pager | measured | frame rows identical (72, 117, 126, 174) | DONE |
| Discover cards | measured | frames 230/384/538/692 identical; chips, author, age and counts now read from the same API fields | DONE / FLOOR |
| Discover icons | measured | 9 668 across four cards -- network images at a different scale from the reference's | OPEN |
| Discover category rail | measured | drawn with marks, 46-pitch rows, retried fetch | DONE |
| Screenshots, empty | measured | 25 600; content 4 935 with the reference's own illustration | DONE / FLOOR |
| Screenshots, populated | not comparable | the reference has no screenshots in this lab | BLOCKED-DATA |
| Screenshot groups / ZIP / drag-drop | store tested, dialog reverted | presses inside the dialog never registered | OPEN |
| Skins, empty | measured | 164 451; saved grid and 4x2 defaults match; preview column open | OPEN |
| Skins, 3D preview overlay layers | not built | the reference's model has hat and jacket layers | OPEN |
| Servers | not comparable | the reference answers "Modrinth App update required" | BLOCKED-REFERENCE |
| Settings, 13 tabs | in flight | a workstream is capturing each tab against its own reference | OPEN |
| Instance page, 5 tabs | in flight | a workstream is taking the reference capture first | OPEN |
| Project page + versions pager | in flight | same | OPEN |
| Account / Follow / Save / Report | no client | BLOCKED-SERVICE |
| Rendering gate | measured | 48.4 MB idle RSS (bound 20, FAIL), 13.0--14.9 ms p95 (bound 16, PASS) | DONE |
| CI | no remote in this environment | BLOCKED-ENVIRONMENT |

## A correction that invalidates part of the earlier record (2026-11-03)

The Settings workstream proved that **`.scratch/cap/ref-settings.png` and `ref-tab-*.png` are not reference
captures**: `ref-tab-14.png` is byte-identical to `ours-settings-open.png`, and ref-settings.png carries this
build's own "Sift Launcher 0.1.62" rail footer. Several settings comparisons in this record were therefore this
build measured against itself, and the "known" gaps they produced (plate 928 against 900, a scrolling rail, missing
body blocks) were artefacts of that.

Genuine reference captures now on file, each taken live from the Modrinth App 0.22.0 AppImage on its own display
with the pointer parked away from the window:

- `ref101-*.png` -- fourteen settings tabs (:101)
- `ref-instance.png`, `ref-tab-{content,files,screenshots,worlds,logs,share}.png` -- the instance page (:102)
- `ref-project.png`, `ref-project-versions.png` -- a project's Description and Versions tabs (:103)

The captures this record's headline numbers rest on were checked the same way and hold: `ab-ref-home.png`,
`ref-discover-fresh.png`, `ref-screenshots.png` and `ref-skins.png` all differ from this build by tens of thousands
of pixels, so none of them can be a capture of it.

## The game actually launches (2026-11-04, verified by driving)

Play on the "Parity Test" instance (Vanilla 26.3, the lab's own) spawned the JVM through this build's own launch
path and Minecraft booted. Its log, `instances/Parity Test/minecraft/logs/latest.log`:

```
[17:12:06] [Datafixer Bootstrap #0/INFO]: 307 Datafixer optimizations took 499 milliseconds
[17:12:10] [Render thread/INFO]: Environment: Environment[discoveryUrl=https://discovery.services... name=PROD]
[17:12:10] [Render thread/INFO]: Setting user: Player
[17:12:11] [Render thread/INFO]: Backend library: LWJGL version 3.4.3+4
[17:12:11] [Render thread/INFO]: SDL [SYSTEM]: App name: Minecraft
[17:12:11] [Render thread/INFO]: SDL [SYSTEM]: App version: 26.3
[17:12:11] [Render thread/ERROR]: Failed to create backend OpenGL
com.mojang.renderpearl.api.device.BackendCreationException: Failed to create window for OpenGL context:
    Couldn't find matching GLX visual
```

So: the instance's own files resolved, the bundled JRE (`java/java-runtime-epsilon`, OpenJDK 25.0.1) ran, the offline
account was accepted, 479 MB of assets were read, and Minecraft's own window was created before its renderer failed.

**Why it stops there**: this lab container has `libGL.so.1` and `libGLX_mesa.so.0` but **no DRI drivers** -- no
`swrast_dri.so`, no llvmpipe -- so there is no matching GLX visual for LWJGL to bind. That is the environment, not
the launcher: the failure is inside Minecraft's own renderer, after our command line already worked.

What this proves and what it does not: instance creation, version resolution, the asset index, the java runtime, the
offline session and the process handling all work. It does **not** prove the game reaches its main menu, which needs
a GL driver this container does not have.

## The game reaches its main menu (2026-11-05, verified by capture)

`game-main-menu.png` is the launcher's own screen with **Minecraft 26.3's title screen up on top of it**: the
"MINECRAFT JAVA EDITION" logo, the splash text, and the mod-pack welcome block "Welcome to Fabulously Optimized!" with
its controls (Narrator, Music & Sounds, Accessibility Settings, Language, Continue).

So the whole chain works end to end: Play on the instance page returns `InstanceAction::Play`, the shell starts the
launch, the engine resolves the instance's files against the 479 MB asset store, the bundled OpenJDK 25.0.1 runs, the
offline session is accepted, and Minecraft reaches the menu a reader would click Play at.

An earlier run the same day stopped at "Failed to create window for OpenGL context: Couldn't find matching GLX visual",
which is this container's missing DRI drivers; the client's own fallback path got past it here. Both outcomes are in the
record: the difference is the client, not this launcher.

**Measurement trap, recorded because it cost an hour**: the first readings after this launch were Home 132 538 and
Discover 272 883. Nothing had regressed -- Minecraft's window was covering the launcher in the capture. With the game
closed the same build measures Home **24 444** and Discover **78 987**. A capture is only a measurement of the launcher
when the launcher is what is on screen.

### Two capture names that lie, found while measuring the library

- **`ref101-library.png` is the Skins page**, not the library: comparing our library against it reads 200 625, which is
  two different pages measured against each other. The file is left where it is, with this note, rather than deleted --
  it is a genuine reference capture of *something*, just not of the page its name claims.
- `105-final-home.png` is the **empty-library Home** ("Welcome to Modrinth", "Create an instance", "Press N to
  quick-create an instance"), which is the state our 24 444 reading was taken in.

So the library's own number has no trustworthy reference at this moment. It is not that the page is close or far; it is
that the pair of states does not exist yet. Naming it here is cheaper than re-learning it next week.

### A third fake reference, caught by hash (2026-11-05)

I started the reference AppImage on display :107 expecting a library page with instances, captured "the reference", and
compared it: **total 0, every slab 0**. A byte-identical result is not a perfect match -- it is the same picture twice.
`md5sum` gives `f496e409...` for both files, and the picture is this build's own library.

What went wrong: the AppImage never put a window on screen. Every `ModrinthApp` process in the container is
`<defunct>`, and the window `xdotool` found was ours. `ref107-lib.png` is deleted rather than kept.

**The rule, now caught three times**: a capture is only a reference if the process that drew it is the reference. Check
the process, not the filename -- and treat "zero differing pixels" as the loudest possible alarm, not a triumph.

### The News card is a rotation, and that is most of the sidebar's difference (found 2026-11-05)

Comparing the instance page against the reference's own (display :101, window owner `Modrinth App`) put **106 588**
differing pixels in the sidebar band alone -- far more than any text rendering can account for. The reason is in the
two captures: at the moment of the reference's capture its News card read **"Join our server"** with a "Get your next
server on Modrinth" banner under it, where this build always draws **"Sync settings across instances"**.

So the card is a **carousel**: the reference cycles articles (the same sidebar showed "Sync settings across instances"
earlier in the session), and this build draws one fixed article. That single difference is behind a large share of the
sidebar figure on **every** page -- Home 10 875, Discover 14 052, the instance page 10 896 -- not three separate defects.

Recorded as the next feature slice: a slide list, a timer, and the same card geometry per slide. It cannot be measured
honestly until the two sides are on the *same* slide, which means the carousel has to exist first.

**What did match** on the instance page, checked in the same pair of captures: the header's icon, name, the
"◇ Fabric 26.3 ⌄ • ⏱ Never played" line, and the Play / gear / kebab cluster; and the five tabs Content, Files, Worlds,
Logs, Share with Content selected.

### The carousel's effect, measured at last on equal terms (2026-11-05)

Both windows are now 1440x900 -- the reference was running at **1280x800** for most of this session, which is why so
many comparisons read high -- and the instance page measures:

| slab | before (1280 vs 1440) | after (1440 vs 1440) |
|---|---:|---:|
| total | 200 965 | **128 308** |
| sidebar | 106 588 | **32 005** |
| titlebar | 11 650 | **2 929** |
| rail | 7 732 | **1 102** |

The sidebar's fall of 74 583 is the News carousel: one rotating card had been standing in for a whole column of defects.

**The content slab (92 272) is a state difference, not a layout one**: this build's instance has a mods list -- 26 entries
with file sizes -- while the reference's is empty and shows "No content installed. Browse or upload projects to get
started." with its Upload files / Browse content pair. Comparing those two panes is comparing two different screens, and
it is recorded that way rather than as a number to chase.

**The lesson of the window size is worth more than any of the numbers**: a capture is only a measurement when the two
windows are the same size *and* in the same state. Both halves of that rule have now been broken once in this record.

### All five instance tabs now verified against the reference's own captures (2026-11-05)

| tab | what was compared | result |
|---|---|---|
| Content | empty state: art, "No content installed", copy, both buttons | matches; art 81x59 ink on both, buttons' marks corrected |
| Files | the root row: home, search, Refresh, split "+" | rebuilt to the reference's arrangement |
| Worlds | empty state: art, "No servers or worlds added", copy | matches |
| Logs | the row above the console and the console's frame | rebuilt: All chip, Live Log select, frame, rule |
| Share | empty state: art, "No friends invited", "You can share this instance with your friends!" | matches |

Each pair was taken at 1440x900 with both windows the same size and the same tab selected, which is the rule this
record has broken twice and now states in every entry.

### The project page, measured against the reference's own capture (2026-11-05)

`total 90 460` -- titlebar 4 882, rail 1 242, content 47 909, sidebar 36 427.

**What matched**, checked line by line in the two captures: the breadcrumb "Discover modpacks > Fabulously Optimized",
the icon, title, "by ..." line and description, the Install and kebab controls, the meta row (downloads, followers,
tags), the three tabs Description / Versions / Gallery, the action row ("How to install", "Discord", "Rate"), the
description text, **the inline gallery image**, and the "Performance" section.

**One difference fixed this round**: the Compatibility card's version chips drew as a run of loose words because their
fill sits a shade off the card's own and they had no ring. They are ringed in `SURFACE_5` now, as the reference's are.

**One difference found and recorded as open**: the reference's project sidebar is *Getting started* then
*Compatibility* then the Upgrade link -- no News card. This build's sidebar shows Compatibility with no Getting started
block above it, so the card sits higher and the panel below it shifts.

### Why the two Getting started cards are not yet one (2026-11-05)

The project page now calls `panel::getting_started_card`; the sidebar inside `panel::draw` still draws its own. That is
not laziness, it is a measurement:

- the sidebar's rows start at `steps_first = getting_started + HEADING_LINE + HEADING_TO_BODY`, i.e. **34** below the
  heading's top;
- the shared card's rows start at `card.top + 34`, and its card top is `getting_started - 16`, i.e. **18** below the
  heading's top.

Sixteen points apart. Driving the sidebar's card into the shared function would move Home's three rows up by 16 and
change a figure that is currently 24 444, so the merge is a decision about which measurement is right -- not a mechanical
edit. The shared function now takes its **fill and ring** from the caller (the sidebar's is the gradient showing through,
the project page's a flat raised surface), which is the part of the difference that *was* mechanical and is now done.

Recorded so the next attempt starts from the numbers rather than from the assumption that the two cards are the same.

### The 16-point question is answered: 58, not 34, and the reference is the same on both pages (2026-11-05)

Measured off the two reference captures, its sidebar card is **identical on Home and on a project page**:

```
heading ink   84..95       (both)
first row    138..155      (both)
second row   187..203      (both)
```

So there was never a page difference to reconcile -- only this build's own two drawings disagreeing. Our Home already
matched; our *project* card drew its rows 24 points high because the shared function put them at `card.top + 34` where
the measurement says `card.top + 58`.

With that corrected the project card reads:

```
ours  heading 85..95   first row 138..153   second row 186..194
```

and the page measures **92 195**, down from 93 047.

The sidebar's own inline drawing stays as it is: it is already within a point of the same measurement, and folding it into
the shared function is now a straight swap rather than a decision -- which is the next step, not this round's.

### The shared card's hover is wired but cannot be seen yet (2026-11-05, open)

`getting_started_card` now takes the pointer and brightens a row under it by the theme's 8% step, which is what the
sidebar's own drawing does. Driving the project page with the pointer held over a row shows **no change at all**:

```
hovered fill (39, 41, 46)   resting fill (39, 41, 46)   -- zero differing pixels down the column
```

The cause is not the tint: the card **registers no widget**, so egui has no reason to repaint when the pointer moves and
the frame that would show the hover is never drawn. The sidebar's card gets away with it because Home repaints constantly
for other reasons.

The fix is to give `getting_started_card` the `ui` rather than a painter and `interact` each row, which is the same
change that would let it report a pressed step and let the sidebar adopt it. Recorded as the next step and **not**
claimed as working.

### Correction: the shared card's hover does render (2026-11-05)

Last round's entry said the hover could not be seen. **That was a sampling error, not a defect.** Scanning the whole
sidebar instead of one column shows the truth:

```
project card, pointer on row 1 vs pointer away: 1 025 differing pixels, first at (1186, 126)
home card,    pointer on row 1 vs pointer away:    40 differing rows at x 1300
```

The rows had simply been sampled at x 1400, past their right edge. With the card now taking the `ui` and registering each
row, the hover renders on both sidebars.

So the two cards are one component in behaviour as well as in shape: same heading, same row pitch, same tick, same
brightening under the pointer. What still differs is only who draws the sidebar's own copy, and `panel::draw` has not
been switched over -- that swap is now a straight replacement with nothing left to reconcile.

### The Skin selector had a search field the reference does not have (2026-11-05)

First real comparison of the Skins page, both windows at 1440x900 with the same rail item selected (the reference's own
tooltip names it "Skin selector", which is how the item was found after two mis-clicks):

- the reference's toolbar is **Skin selector, then the Wide/Slim toggle, then Add skin** -- and **no search field**;
- this build drew a 320-point "Search skins..." plate between the heading and the tabs.

The plate is gone; the tabs now start at the row's own left edge. The source already knew -- its own comment reads "the
reference has no skins search field" -- and the field was invented anyway.

**The page still measures about 189 000**, and that figure is almost all *state*: the reference's saved-skins list is empty
while this lab's holds three (hero, nightcap, probe-face), so the two grids are different pictures. What is comparable --
the header, the toggle, the Add skin card, the Default skins row -- now lines up.

### The Skin selector's marks, against the reference's own capture (2026-11-05)

- **The in-use badge is a light disc**, not the brand-green circle this build drew: the reference's capture shows a pale
  circle with a dark tick at the card's top-right. Changed to `CONTRAST` with a `BUTTON_BG` tick.
- The **search field is gone** (previous round), and the tabs now start at the row's left edge.

Page totals for this pair (both 1440x900, same rail item): **188 034 -> 189 047** across the two changes. The figure is
dominated by what the two libraries hold -- the reference has no saved skins, this lab has three -- so it is recorded as a
state difference and not chased.

### The library, measured with matched state at last (2026-11-05)

A data directory holding **exactly one instance** (`/tmp/matchdata`) against the reference's one:

| slab | pixels |
|---|---:|
| total | **89 051** |
| content | 53 788 |
| sidebar | 32 005 |
| titlebar | 2 229 |
| rail | 1 029 |

That is the first library number in this record where both sides hold the same *kind* of thing. Three defects it exposed,
all fixed:

1. **"New group" had no plus and no ring** -- the reference's pair is an outlined "+ New group" beside a filled
   "+ New instance", and ours was two bare labels.
2. **"New instance" had no plus** either; both buttons lead with one now.
3. The field's width is closer (600 against the reference's ~640) but not equal -- recorded, not claimed.

What still differs on this page is the instance itself: its name ("Instance 3" against "Fabric 26.3"), its loader line
("Vanilla 26.3" against "Fabric 26.3") and its card art, which this lab's instance does not have. Those are data, and the
record keeps them counted as data rather than chased as pixels.

### Skins, measured with matched libraries (2026-11-05)

With no saved skins on either side (`/tmp/matchdata`), the page measures **155 619** -- titlebar 2 844, rail 1 049,
content 119 092, sidebar 32 634.

The structure lines up element for element: "Skin selector", the collapsed "Saved skins" section with its chevron, the
"Add skin / Drag and drop" card, the preview figure at the left, and the "Default skins" row whose first card carries the
pale selection disc.

**The preview is identical, not merely similar**: its ink spans x 156..259, y 289..519 in the reference and x 159..259,
y 290..519 here -- within three pixels on every edge. What reads as "ours is smaller" in a resized crop is the *figure's
art*, which is the default skin itself, not the frame around it.

The content slab is therefore almost entirely **skin artwork**: four default skins drawn from different files. That is
data, not drawing, and is recorded as such.

### The sidebar's 32 000 was one missing picture (2026-11-05)

The sidebar slab sat at ~32 000 on every page and would not move. It is not text and not the carousel's phase: it is the
news plate, **272x120 = 32 640 pixels**, and this build shipped one plate for both articles while the reference draws a
different picture for each.

The second plate is now cropped from the reference's own render (1156,350 to 1428,470, the moment its card read "Joining
your server just got easier") and the card picks the plate by slide.

Measured on the Screenshots page, three samples six seconds apart:

| sample | total | sidebar |
|---|---:|---:|
| t=0, same article | **22 351** | **14 213** |
| t=1, same article | **22 351** | **14 213** |
| t=2, different articles | 40 143 | 32 005 |

So the ~18 000 that vanished is the plate, and what remains of the sidebar (14 213) is the card's text and the hosting
banner. **The lesson is the same one this record keeps teaching**: a stubborn slab is worth measuring against the thing
that occupies it, not against the number.

### Where the sidebar's remaining 14 213 actually is (2026-11-05)

With the carousels on the same article, the sidebar's difference bands like this:

| band | pixels |
|---|---:|
| Getting started (48..290) | 4 100 |
| News heading (290..380) | 246 |
| News card (380..560) | 3 234 |
| **News copy, date and upgrade row (560..700)** | **7 994** |
| Hosting banner art (700..900) | 750 |

The largest band is not the banner at all -- it is the news card's **text block**: this build's copy wraps at different
words than the reference's, and its second slide carries no date where the reference's does (the reference's own layout
puts that date under the "Upgrade to Modrinth+" link, which is why its row reads as overlapping).

Recorded rather than tuned blind: the two ways to close it are to match the copy column's width so the wraps agree, and to
give the second slide its date, and neither has evidence behind it yet beyond this band's existence.

### A negative result, kept: the copy's wrap width is not the column (2026-11-05)

The band breakdown put 7 994 pixels in the news card's text block, and the obvious suspect was the wrap: the reference's
copy ink ends at x 1413 and this build's at 1423.

Narrowing the layout width by those ten points changed **nothing at all** -- the whole page read 22 351 with a 14 213
sidebar before the change and byte-for-byte the same after it, across three samples. So the line breaks are not set by
that argument, and the change is reverted rather than left in the source as an unexplained "−10".

What that rules out is worth as much as a fix would be: the difference in that band is in the *text itself* or in how its
galley is built, not in the width passed to it. The next attempt should start by printing the copy's galley size from
both sides rather than guessing at which number feeds it.

### The 7 994 band is an offset and one of the reference's own artefacts (2026-11-05)

Following the note from the previous round, both copies were cropped at 1:1 with the same article up. They read **the
same words with the same breaks**:

```
Share your server with friends directly
in the app. When they accept, we'll
create an instance for them that's
managed by your server.
```

and their line pitch matches too (17-18 points on both, glyph ink 10-11 points). So the wrap was never the problem and the
width change that did nothing was right to be reverted.

What is left in that band is two things:

1. **A vertical offset** -- the reference's copy begins about 18 points lower than this build's, because its two-line title
   takes more room above it than the row plan reserves here.
2. **The reference's own overlap**: its "Upgrade to Modrinth+" row sits *on top of* the copy's last line ("managed by your
   server." runs through the link), which is a layout bug in the reference, visible in its own capture.

Reproducing (2) would mean deliberately drawing text over a link. That is a choice about faithfulness, not a measurement,
and it is recorded here rather than made silently.

(1) is a real, closable difference and the next target: the row plan's title allowance against a two-line title.

### The title's height now feeds the row plan, and why that was not observable yet (2026-11-05)

`news_rows` reserved a single `TITLE_LINE` (20 points) for the title whatever it wrapped to. "Joining your server just
got easier" is a two-line title, so the reference's copy begins a line lower than a one-line title's -- the 18-point
offset the band breakdown found. The plan now takes the title galley's own height (`title_height.max(TITLE_LINE)`), and
both tests that build a plan pass `TITLE_LINE`, keeping their single-line case exact.

**It could not be observed in seven samples**: every sampled moment where the two carousels were on the same article read
**14 213** for the sidebar, never another value, which means the matches landed on the one-line slide where the change is a
no-op by construction. Recorded as *implemented and reasoned*, not as *verified* -- and the next session should hold both
apps on the two-line article deliberately (by freezing the clock) before claiming it.

The alternative reading -- that the second slide's plate never draws -- would look the same from outside, so the same
experiment settles both.

### Resolved: the second slide's plate does draw (2026-11-05)

The open alternative from last round -- that the second plate never draws, which would look identical from outside -- is
**ruled out by measuring our own app against itself**:

```
art region (1156..1428 x 350..470) sampled 11 seconds apart: 31 762 of 32 640 pixels differ
```

Two different pictures, one per slide. So the carousel's plate selection works, and the sidebar's constant **14 213** on
every matched sample means those samples simply landed on the one-line article, where the title-height change is a no-op.

What remains unknown is narrower than before: how the two apps read when both are on the *two-line* article, which needs
both pinned to it rather than sampled. That is a one-line change to the slide clock when someone next wants the number --
`ui.input().time` replaced by a fixed phase -- and until then this note stands as the honest state of it.

### The sidebar's remainder is glyph rasterisation, proved by a threshold sweep (2026-11-05)

With the carousel **pinned** to the two-line article (`SIFT_NEWS_SLIDE=1`), the reference caught on the same article,
and the art regions **provably identical** (0 differing pixels in 1156..1428 x 350..470), the sidebar still reads 14 213.

Sweeping the difference threshold over the sidebar column alone:

| threshold | differing pixels |
|---:|---:|
| 40 | 16 324 |
| 80 | 11 137 |
| 120 | 8 173 |
| 160 | **5 235** |

A **layout** difference behaves nothing like this: a block shifted by a few points stays roughly constant until the
threshold is enormous. A curve that halves as the tolerance doubles is two rasterisers disagreeing about glyph edges --
which is the same floor the Home page showed at the very start of this work (31 171 at 40, 13 215 at 120, 5 549 at 160).

So the sidebar is **done to its floor**, and the standing conclusion for every remaining slab of this kind is the same one:
what is left is text, not structure. Recorded with the numbers so that the next session does not spend itself re-discovering
it, and so that no one mistakes 14 213 for something fixable.

`SIFT_NEWS_SLIDE` is now a supported affordance rather than a throwaway: measuring a rotating card against another app's
means pinning both, and this session spent three rounds learning that.
