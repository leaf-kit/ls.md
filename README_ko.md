<p align="center">
  <img src="images/logo.png" alt="lsmd 로고" width="240" />
</p>

# lsmd — **L**ist **M**ark**d**own

[![Release](https://img.shields.io/github/v/release/leaf-kit/ls.md?label=release)](https://github.com/leaf-kit/ls.md/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Homebrew](https://img.shields.io/badge/homebrew-leaf--kit%2Flsmd-yellow.svg)](https://github.com/leaf-kit/homebrew-lsmd)
[![brew install](https://img.shields.io/badge/brew%20install-lsmd-success.svg)](https://github.com/leaf-kit/homebrew-lsmd)

마크다운 구조를 이해하는 디렉터리 목록 도구.

> **v0.2.1 출시** — [GitHub Release](https://github.com/leaf-kit/ls.md/releases/tag/v0.2.1) | [Homebrew Tap](https://github.com/leaf-kit/homebrew-lsmd)
>
> ```bash
> brew tap leaf-kit/lsmd && brew install lsmd
> ```

**lsmd**는 `ls`의 동반자 도구로, 마크다운을 매일 다루는 개발자, 테크니컬 라이터, PKM 사용자를 위해 만들어졌습니다. YAML 프론트매터를 파싱하고, 헤딩을 추출하고, 텍스트 파일을 미리 보여주며, 컬러 태그 배지를 렌더링합니다 — 모두 한 줄에, 하나의 명령어로.

Rust로 작성되어 빠르고 안전합니다. LTO 최적화. 런타임 의존성 없음. 단일 바이너리 배포.

📖 [English README](README.md)

---

## 왜 lsmd인가? 🤔

### 문제점

`ls`로는 파일 이름 목록만 보입니다:

```
% ls
api-design.md    debugging-checklist.txt  markdown-style.md     quick-reference.txt
cli-ux-tips.md   git-workflow.md          project-kickoff.md    rust-error-handling.md
```

> `api-design.md`에 어떤 태그가 있는지? `project-kickoff.md`는 언제 작성되었는지?
> `quick-reference.txt`는 무엇에 관한 내용인지? **각 파일을 열어봐야 알 수 있습니다.**

### 해결책 💡

**lsmd**는 파일 내용을 읽어 답을 보여줍니다 — 파일을 열지 않고도:

<p align="center">
  <img src="images/lsmd.png" alt="lsmd 기본 목록" width="100%" />
</p>

> **`.md` 파일** → $\color{green}{\textsf{제목}}$ · $\color{gray}{\textsf{날짜}}$ · `컬러 태그 배지`
>
> **`.txt` 파일** → $\color{gray}{\textsf{첫 줄 미리보기 (흐림, 최대 60자)}}$
>
> **디렉터리** → $\color{dodgerblue}{\textsf{▸ 파란색 굵은 글씨, 항상 최상단}}$

한눈에 각 문서의 주제, 작성 시기, 다루는 토픽을 파악할 수 있습니다.

### PKM(개인 지식 관리)을 위한 도구 🧠

**Obsidian**, **Logseq**, **Dendron** 또는 일반 마크다운 파일로 개인 지식 베이스를 관리한다면, **lsmd**는 터미널에서의 동반자입니다:

- 🗂️ **터미널에서 vault 탐색** — GUI 없이 제목, 날짜, 태그를 확인
- 🔍 **태그로 노트 검색** — `grep`과 파이프하여 `#rust`나 `#meeting` 태그 노트를 즉시 검색
- 👀 **열기 전에 스캔** — 100개 파일의 내용을 열지 않고도 파악
- ✅ **한눈에 점검** — 프론트매터와 태그가 제대로 있는지 빠르게 확인
- 🔗 **Unix 도구와 결합** — `grep`, `awk`, `sort`로 지식 베이스 전체를 검색

> *파일을 나열하지 마세요. 의미를 나열하세요.*

---

## 기능 ✨

| 카테고리 | 기능 |
|----------|------|
| 📋 **콘텐츠** | YAML 프론트매터 파싱 (제목 · 날짜 · 태그) |
| 🏷️ **태그** | 해시 기반 컬러 태그 배지 (같은 태그 = 같은 색상) |
| 📝 **폴백** | H1 헤딩 → 첫 본문 줄 → 표시 없음 (단계적 폴백) |
| 📄 **텍스트** | `.txt` 첫 줄 미리보기 (정제, 60자 말줄임) |
| 🎨 **색상** | ANSI-256 TrueColor 팔레트 |
| 📐 **정렬** | 22자 이름 컬럼, 긴 이름 `…` 축약 (확장자 보존) |
| 📊 **헤더** | 디렉터리 경로 + 요약 카운트 (dirs, files, md, txt) |
| 🔒 **권한** | 컬러 `rwx` 표시: $\color{green}{\textsf{r}}$$\color{goldenrod}{\textsf{w}}$$\color{darkred}{\textsf{x}}$ |
| 🌈 **크기** | $\color{wheat}{\textsf{< 1 MB}}$ · $\color{lightsalmon}{\textsf{< 1 GB}}$ · $\color{orange}{\textsf{≥ 1 GB}}$ |
| 📅 **날짜** | $\color{green}{\textsf{< 1시간}}$ · $\color{springgreen}{\textsf{< 1일}}$ · $\color{darkcyan}{\textsf{이전}}$ |
| 📁 **정렬** | 디렉터리 우선; 이름, 크기, 수정일, 유형별 정렬 |
| 🔤 **제목 모드** | `-t`로 첫 `#` 헤딩만 표시 |
| 📂 **필터** | `-m` 마크다운/텍스트만, `-a` 숨김 파일 표시 |
| ⚡ **성능** | Rust + LTO, 단일 바이너리, 의존성 없음 |

---

## 설치 📦

### Homebrew (macOS)

```bash
brew tap leaf-kit/lsmd
brew install lsmd
```

### 소스에서 빌드

```bash
git clone https://github.com/leaf-kit/ls.md.git
cd ls.md
cargo build --release
cp target/release/lsmd /usr/local/bin/
```

또는 인터랙티브 빌드 스크립트 사용 (빌드 전 테스트 자동 실행):

```bash
./build.sh
```

## 업데이트

| 방법 | 명령어 |
|------|--------|
| Homebrew | `brew upgrade lsmd` |
| 소스 | `git pull && cargo build --release && cp target/release/lsmd /usr/local/bin/` |

## 삭제

| 방법 | 명령어 |
|------|--------|
| Homebrew | `brew uninstall lsmd && brew untap leaf-kit/lsmd` |
| 수동 | `rm /usr/local/bin/lsmd` |

---

## 명령어 및 출력 예시 📸

모든 예시는 프로젝트에 포함된 `playground/` 디렉터리에서 실제 실행한 결과입니다.

### 1. 기본 목록 — `lsmd`

<p align="center">
  <img src="images/lsmd.png" alt="lsmd 기본 목록" width="100%" />
</p>

<details>
<summary>📋 텍스트 출력 (클릭하여 펼치기)</summary>

```
% lsmd playground
 /path/to/playground  (2 dirs, 13 files, 7 md, 3 txt)

 ▸ best-practices
 ▸ docs
 ◦ app.py
 ◆ blog-post.md            Getting Started with Rust · 2026-03-15 ·  rust   programming   tutorial
 ◆ broken-yaml.md          Broken YAML Frontmatter Test
 ◦ config.yaml
 ◆ empty.md
 ○ empty.txt
 ○ long-line.txt           This is a very long first line that should be truncated wit…
 ◆ meeting-notes.md        Team Meeting Notes · 2026-04-01 ·  meeting   planning
 ◆ no-frontmatter.md       Simple Document
 ○ notes.txt               Quick notes from today's brainstorming session about the ne…
 ◦ sample.json
 ◆ short.md                Short
 ◆ very-long-filename….md  Long Name Test · 2026-04-05 ·  test

 Total: 15 items, 2.4 KB
```

</details>

> **범례:** `▸` 디렉터리 · `◆` 마크다운 · `○` 텍스트 · `◦` 기타

### 2. 상세 포맷 — `lsmd -l`

<p align="center">
  <img src="images/lsmd-l.png" alt="lsmd -l 상세 포맷" width="100%" />
</p>

> **권한** · **아이콘** · **크기** (크기별 색상) · **날짜** (최근도별 색상) · **파일명** · **메타데이터** 표시

### 3. 숨김 파일 표시 — `lsmd -a`

<p align="center">
  <img src="images/lsmd-a.png" alt="lsmd -a 숨김 파일" width="100%" />
</p>

> 도트파일과 숨김 디렉터리를 표시합니다.

### 4. 마크다운만 표시 — `lsmd -m`

<p align="center">
  <img src="images/lsmd-m.png" alt="lsmd -m 마크다운만" width="100%" />
</p>

> `.md`와 `.txt` 파일만 필터링합니다. 디렉터리는 항상 포함됩니다.

### 5. 크기순 정렬 — `lsmd -s size`

<p align="center">
  <img src="images/lsmd-s_size.png" alt="lsmd -s size 크기순 정렬" width="100%" />
</p>

> 파일 크기순으로 정렬합니다 (작은 파일부터).

### 6. 제목만 표시 — `lsmd -t`

`.md` 파일에서 프론트매터 세부 정보 없이 첫 번째 `#` 헤딩만 표시합니다:

```
% lsmd playground/best-practices -t
 /path/to/playground/best-practices  (8 files, 6 md, 2 txt)

 ◆ api-design.md           RESTful API Design Principles
 ◆ cli-ux-tips.md          CLI UX Design Tips
 ○ debugging-checkli….txt  Step-by-step debugging checklist for production incidents
 ◆ git-workflow.md         Git Workflow Guide
 ◆ markdown-style.md       Markdown Writing Style Guide
 ◆ project-kickoff.md      Project Kickoff Checklist
 ○ quick-reference.txt     Common terminal shortcuts and commands for daily developmen…
 ◆ rust-error-handling.md  Rust Error Handling Patterns

 Total: 8 items, 5.0 KB
```

> 대규모 vault에 적합한 깔끔한 제목 스캔. `-l`과 결합하여 크기와 날짜도 함께 볼 수 있습니다.

### 7. 모범 예제 — 프론트매터 포함 예시

```
% lsmd playground/best-practices
 /path/to/playground/best-practices  (8 files, 6 md, 2 txt)

 ◆ api-design.md           RESTful API Design Principles · 2026-03-15 ·  api   rest   design
 ◆ cli-ux-tips.md          CLI UX Design Tips · 2026-04-03 ·  cli   ux   design
 ○ debugging-checkli….txt  Step-by-step debugging checklist for production incidents
 ◆ git-workflow.md         Git Workflow Guide · 2026-03-20 ·  git   workflow   collaboration
 ◆ markdown-style.md       Markdown Writing Style Guide · 2026-03-10 ·  markdown   writing   documentation
 ◆ project-kickoff.md      Project Kickoff Checklist · 2026-04-01 ·  project   checklist   onboarding
 ○ quick-reference.txt     Common terminal shortcuts and commands for daily developmen…
 ◆ rust-error-handling.md  Rust Error Handling Patterns · 2026-03-28 ·  rust   error-handling   patterns

 Total: 8 items, 5.0 KB
```

---

## 옵션 레퍼런스 ⚙️

| 옵션 | 단축 | 설명 |
|------|------|------|
| `--all` | `-a` | 숨김 파일(도트파일) 표시 |
| `--long` | `-l` | 상세 포맷: 권한, 크기, 날짜, 메타데이터 |
| `--no-color` | | ANSI 색상 비활성화 |
| `--sort <FIELD>` | `-s` | 정렬: `name`, `size`, `modified`, `type` |
| `--reverse` | `-r` | 역순 정렬 |
| `--md-only` | `-m` | `.md`와 `.txt` 파일만 표시 |
| `--title` | `-t` | `.md` 파일의 첫 `#` 헤딩만 표시 |

---

## 색상 체계 🎨

lsmd는 ANSI-256 TrueColor 팔레트를 사용합니다:

### 파일 유형별 색상

| 요소 | 색상 | 예시 |
|------|------|------|
| 디렉터리 | $\color{dodgerblue}{\textsf{DodgerBlue 굵음}}$ | `▸ best-practices` |
| `.md` 파일 | $\color{green}{\textsf{연두색}}$ | `◆ blog-post.md` |
| `.txt` 파일 | $\color{wheat}{\textsf{따뜻한 흰색}}$ | `○ notes.txt` |
| 기타 파일 | $\color{goldenrod}{\textsf{노란색}}$ | `◦ app.py` |
| 숨김 파일 | $\color{gray}{\textsf{회색}}$ | `◆ .hidden-file.md` |
| 실행 파일 | $\color{limegreen}{\textsf{초록 굵음}}$ | `◦ script.sh` |

### 상세 포맷 컬럼

| 컬럼 | 색상 규칙 |
|------|----------|
| 권한 `r` | $\color{green}{\textsf{초록}}$ |
| 권한 `w` | $\color{goldenrod}{\textsf{노랑}}$ |
| 권한 `x` | $\color{darkred}{\textsf{빨강}}$ |
| 권한 `-` / `.` | $\color{gray}{\textsf{회색}}$ |
| 크기 < 1 MB | $\color{wheat}{\textsf{Wheat}}$ |
| 크기 < 1 GB | $\color{lightsalmon}{\textsf{LightSalmon}}$ |
| 크기 ≥ 1 GB | $\color{orange}{\textsf{Orange}}$ |
| 날짜 < 1시간 | $\color{limegreen}{\textsf{Green}}$ |
| 날짜 < 1일 | $\color{springgreen}{\textsf{SpringGreen}}$ |
| 날짜 이전 | $\color{darkcyan}{\textsf{DarkCyan}}$ |

---

## 콘텐츠 미리보기 정책 📖

lsmd는 `.md`와 `.txt` 파일에서 한 줄 요약을 추출합니다. 미리보기 텍스트는 **정제** 처리됩니다 — 특수 문자가 제거되고 읽을 수 있는 텍스트(알파벳, 한글, CJK, 기본 구두점)만 유지됩니다.

### `.md` 미리보기 우선순위

| 우선순위 | 소스 | 표시 방식 |
|---------|------|----------|
| 1 | YAML 프론트매터 | `제목` · `날짜` · 컬러 태그 배지 |
| 2 | 첫 `# H1` 헤딩 | 흐린 헤딩 텍스트 |
| 3 | 첫 본문 줄 | 흐린 내용 (코드 펜스, `---` 건너뜀) |
| 4 | 깨진 YAML | #2 또는 #3으로 폴백 |
| 5 | 빈 파일 | 파일명만 표시 |

### `.txt` 미리보기

| 우선순위 | 소스 | 표시 방식 |
|---------|------|----------|
| 1 | 첫 의미있는 줄 | 흐린 표시, 정제됨, 최대 60자 + `…` |
| 2 | 빈 파일 | 파일명만 표시 |

---

## 파이프 연동 (`|`) 🔧

lsmd는 파이프 시 ANSI 색상을 자동 비활성화하여 `grep`, `awk`, `wc`, `sort`, `sed`, `xargs`와 안전하게 사용할 수 있습니다.

### 유용한 파이프 레시피

```bash
# 태그로 파일 검색
lsmd playground/best-practices | grep "rust"

# "design" 태그 문서 찾기
lsmd playground/best-practices | grep "design"

# 마크다운 파일 수 세기
lsmd playground/best-practices | grep "\.md" | wc -l

# 파일명만 추출
lsmd playground/best-practices | awk '{print $1}'

# 문서 제목만 추출
lsmd playground/best-practices | grep "·" | cut -d'·' -f1 | sed 's/^[[:space:]]*[^ ]* *//'

# 태그 빈도 분석
lsmd playground/best-practices | grep "·" | sed 's/.*·//' | grep -oE '[a-z][-a-z]*' | sort | uniq -c | sort -rn

# 파일별 줄 수
lsmd playground/best-practices | awk '{print $1}' | sed 's|^|playground/best-practices/|' | xargs wc -l
```

> **참고:** 파이프 시 색상 자동 비활성화. 파일명은 첫 번째 필드. 프론트매터 필드는 `·`로 구분. 출력은 UTF-8.

---

## 플레이그라운드 🎮

프로젝트에 포함된 `playground/` 디렉터리에서 모든 기능을 테스트할 수 있습니다:

```
playground/
├── best-practices/          # 풍부한 프론트매터 & 태그를 가진 예제
│   ├── api-design.md        # 태그: api, rest, design
│   ├── cli-ux-tips.md       # 태그: cli, ux, design
│   ├── git-workflow.md      # 태그: git, workflow, collaboration
│   ├── markdown-style.md    # 태그: markdown, writing, documentation
│   ├── project-kickoff.md   # 태그: project, checklist, onboarding
│   ├── rust-error-handling.md
│   ├── debugging-checklist.txt
│   └── quick-reference.txt
├── docs/guide.md
├── blog-post.md             # 프론트매터: 제목 + 날짜 + 태그
├── meeting-notes.md         # 프론트매터: 제목 + 날짜 + 태그
├── no-frontmatter.md        # H1 헤딩만 (폴백 테스트)
├── broken-yaml.md           # 깨진 YAML (에러 처리 테스트)
├── empty.md / empty.txt     # 엣지 케이스: 빈 파일
├── notes.txt / long-line.txt
├── app.py / config.yaml / sample.json
└── short.md / very-long-filename-example.md
```

---

## 엣지 케이스 🛡️

| 시나리오 | 동작 |
|---------|------|
| 빈 파일 | 파일명만 표시, 크래시 없음 |
| 깨진 YAML 프론트매터 | H1 또는 본문 텍스트로 폴백 |
| 매우 긴 파일명 | `…`로 축약, 확장자 보존 |
| 존재하지 않는 경로 | 명확한 에러 메시지 |
| 권한 오류 | 조용히 건너뜀 |
| 파일 경로 (디렉터리 아님) | 명확한 에러 메시지 |

---

## 관련 프로젝트 🔗

| 프로젝트 | 설명 | 차이점 |
|---------|------|--------|
| [**gmd**](https://github.com/leaf-kit/g.md) | Grep Markdown — 구조 인식 검색 | 검색 vs 목록 |
| [**lsd**](https://github.com/lsd-rs/lsd) | LSDeluxe — 아이콘 있는 모던 `ls` | 파일 유형 인식, 콘텐츠 비인식 |
| [**eza**](https://github.com/eza-community/eza) | 모던 `ls` 대체 | 메타데이터 중심, 마크다운 비인식 |

> lsmd는 `.md`와 `.txt` 파일 **내부를 읽어** 구조화된 메타데이터를 인라인으로 표시하는 유일한 `ls` 스타일 도구입니다.

---

## 피드백 & 기여 💬

기여, 이슈, 기능 요청을 환영합니다. lsmd가 유용하다면 저장소에 별표를 달아주세요 — 다른 사용자가 프로젝트를 발견하는 데 도움이 됩니다.

[github.com/leaf-kit/ls.md/issues](https://github.com/leaf-kit/ls.md/issues)

## 문서

- [English README](README.md) — 영문 문서
- [한국어 README](README_ko.md) — 본 문서

## 라이선스

[MIT](LICENSE)
