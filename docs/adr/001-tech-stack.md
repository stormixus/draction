# ADR-001: 기술 스택 및 대상 플랫폼 선택

## 상태

제안됨 (Proposed)

---

## 배경 (Context)

**Draction**은 데스크탑 오버레이 애플리케이션으로, 다음 기능을 네이티브 수준으로 지원해야 한다.

- 투명 배경의 항상-최상단(always-on-top) 오버레이 창
- 파일 및 콘텐츠의 드래그 앤 드롭 수신/처리
- 로컬 SQLite 데이터베이스 연동
- 워크플로우 엔진 (작업 자동화 파이프라인)
- 로컬 HTTP / WebSocket 서버 (외부 도구 및 OpenClaw 연동용)

**OpenClaw**는 AI/UX 브레인 역할을 하며, Draction이 노출하는 로컬 API를 통해 연결된다. OpenClaw 자체는 별도 프로세스(또는 서비스)로 동작하며, 두 컴포넌트 간 통신은 HTTP REST 또는 WebSocket으로 이루어진다.

**초기 타깃 플랫폼**: macOS (v0.1). 향후 Windows / Linux 지원 여부는 기술 스택 선택에 영향을 미친다.

---

## 결정 (Decision)

### 평가 기준

| 기준 | 가중치 | 설명 |
|------|--------|------|
| 오버레이 구현 | 높음 | 투명 창, always-on-top, 시스템 레벨 렌더링 |
| 파일 시스템 접근 | 높음 | 로컬 파일 읽기/쓰기, 경로 접근 권한 |
| 네이티브 드래그 앤 드롭 API | 높음 | OS 드래그 소스로부터 파일/텍스트 수신 |
| 빌드 산출물 크기 | 중간 | 설치 파일 용량, 런타임 번들 포함 여부 |
| 성능 | 중간 | 렌더링 효율, 메모리 사용량, 시작 시간 |
| 크로스 플랫폼 잠재성 | 중간 | macOS 이후 Windows/Linux 포팅 난이도 |

---

### 옵션 1: Electron + TypeScript

**개요**: Chromium 렌더러 + Node.js 백엔드를 하나의 패키지로 묶는 성숙한 데스크탑 프레임워크.

**장점**
- `BrowserWindow` API로 투명 창, always-on-top, frameless 창 구현이 표준화되어 있음
- Node.js 내장으로 로컬 HTTP/WS 서버, SQLite(better-sqlite3 등) 즉시 사용 가능
- TypeScript 생태계 완전 활용, 풍부한 커뮤니티 및 레퍼런스
- 드래그 앤 드롭: HTML5 DnD API + Electron IPC로 OS 레벨 드롭 이벤트 처리 가능
- 크로스 플랫폼(macOS / Windows / Linux) 단일 코드베이스

**단점**
- 번들 크기 과대: Chromium 포함으로 설치 파일 100MB+ 예상
- 메모리 사용량 높음 (Chromium 프로세스 상시 기동)
- 오버레이 특유의 시스템 이벤트(클릭-스루, 완전 투명 레이어 등) 처리에 플랫폼별 네이티브 모듈 추가 필요할 수 있음

**크로스 플랫폼**: 우수 (공식 지원)

---

### 옵션 2: Tauri + Rust / TypeScript

**개요**: OS 네이티브 WebView(macOS: WKWebView)를 UI 레이어로, Rust를 백엔드 코어로 사용하는 경량 프레임워크.

**장점**
- 번들 크기 최소: 런타임 WebView는 OS 내장, Rust 바이너리 수 MB 수준
- Rust 백엔드에서 SQLite(rusqlite), HTTP 서버(axum/warp), 워크플로우 엔진 고성능 구현 가능
- `tauri::window` API로 투명 창, always-on-top, 데코레이션 없는 창 지원
- 드래그 앤 드롭: Tauri v2에서 파일 드롭 이벤트 공식 지원
- 메모리 효율 우수

**단점**
- Rust 학습 곡선: 팀 역량에 따라 초기 개발 속도 저하 가능
- WKWebView(macOS) / WebView2(Windows) 간 렌더링 차이로 크로스 플랫폼 UI 검증 필요
- Tauri v2는 아직 생태계가 Electron 대비 작음
- 복잡한 오버레이 동작(클릭-스루, 마우스 이벤트 패스스루)은 Rust FFI / macOS API 직접 호출 필요할 수 있음

**크로스 플랫폼**: 양호 (공식 지원, 단 WebView 차이 존재)

---

### 옵션 3: Swift / AppKit (macOS 전용)

**개요**: macOS 네이티브 UI 프레임워크로 NSWindow, NSPanel 등을 직접 제어.

**장점**
- 투명 창, always-on-top, 클릭-스루 등 오버레이 기능을 OS API 수준에서 완전 제어
- 네이티브 드래그 앤 드롭(NSDraggingDestination) 완전 지원
- 성능 및 메모리 효율 최고 수준
- App Store 배포, Hardened Runtime, Notarization 공식 지원

**단점**
- macOS 전용: Windows / Linux 이식 불가
- Swift/AppKit 전문 인력 필요
- 로컬 HTTP/WS 서버, SQLite 연동은 직접 구현 또는 Swift 패키지 의존
- UI 레이어를 TypeScript/React로 구성할 수 없음 (WebView 삽입 가능하나 복잡도 증가)

**크로스 플랫폼**: 불가

---

### 서버 런타임 옵션

Draction 내부의 로컬 HTTP / WebSocket 서버 구현체 선택.

| 런타임 | 적합 프레임워크 | 비고 |
|--------|----------------|------|
| Node.js | Fastify, Express | Electron 선택 시 번들 내 자연스럽게 포함 |
| Rust | axum, warp | Tauri 선택 시 백엔드 코어와 일체화 |
| Go | net/http, gin | 독립 바이너리로 사이드카 배포 가능, Electron/Tauri 모두와 조합 가능 |

---

### 데이터베이스

- **SQLite 사용 확정** (로컬 영구 저장소)
- ORM / 드라이버는 선택된 런타임에 따라 결정:
  - Node.js: `better-sqlite3`, `Drizzle ORM`, `Prisma`
  - Rust: `rusqlite`, `sqlx`
  - Go: `modernc.org/sqlite`, `GORM`

---

### 패키지 매니저 및 모노레포

Draction + OpenClaw 두 컴포넌트를 단일 저장소에서 관리할 경우:

- **pnpm workspaces** (Node.js 중심 스택): 빠른 설치, 디스크 효율, workspace 프로토콜
- **Cargo workspaces** (Rust 중심 스택): Rust 크레이트 공유 및 빌드 캐시
- **Nx / Turborepo** (혼합 스택): 언어 무관 태스크 오케스트레이션, 증분 빌드

모노레포 레이아웃 예시 (Tauri 기준):

```
draction/
├── apps/
│   ├── draction/        # Tauri 앱 (Rust + TypeScript)
│   └── openclaw/       # AI/UX 서비스
├── packages/
│   ├── shared-types/   # 공유 타입 정의
│   └── api-client/     # Draction HTTP 클라이언트
└── docs/
    └── adr/
```

---

### 최종 결정 (TBD)

> 이 섹션은 팀 검토 후 채워진다.

현재 평가 중인 우선 후보:

- **1순위 검토**: Tauri v2 + Rust 백엔드 + TypeScript 프론트엔드
  - 근거: 오버레이 기능 지원, 경량 번들, Rust 성능, 크로스 플랫폼 잠재성
- **2순위 검토**: Electron + TypeScript
  - 근거: 성숙한 생태계, 빠른 초기 개발 속도, 풍부한 레퍼런스

v0.1 타깃은 **macOS**이므로, 오버레이 동작 검증을 위한 PoC(개념 증명)를 두 옵션으로 각각 진행한 후 결정한다.

---

## 결과 (Consequences)

> 결정 확정 후 기술한다.

- 선택된 프레임워크가 Draction 오버레이 구현에 미치는 영향
- 서버 런타임 선택이 OpenClaw 연동 방식에 미치는 영향
- 모노레포 구성 및 CI/CD 파이프라인에 미치는 영향
- 향후 Windows / Linux 지원 시 예상 마이그레이션 비용

---

*작성일: 2026-03-04*
*작성자: TBD*
*검토자: TBD*
