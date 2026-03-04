---
provider: "gemini"
agent_role: "designer"
model: "gemini-3-flash-preview"
prompt_id: "da1a8dcd"
used_fallback: true
fallback_model: "gemini-3-flash-preview"
timestamp: "2026-03-04T13:13:14.120Z"
---

I will start by investigating the project's existing technical decisions and framework choices by reading the ADR and checking for configuration files.
## Design Implementation: Draction (Drop + Action)

**Aesthetic Direction:** **"Interactive Toolmate"** - 단순한 유틸리티를 넘어 사용자의 데스크탑에서 살아 움직이는 듯한 느낌을 주는 '반려 도구' 지향. 귀엽고 위트 있는 애니메이션(Playful)과 정갈하고 전문적인 관리 패널(Professional)의 조화를 추구합니다.
**Framework:** **Tauri v2 + React (TypeScript) + Tailwind CSS**

### 1. Overlay Character Design Concept (캐릭터 오버레이 디자인)

*   **비주얼 스타일**: **Lottie(Vector) 기반 애니메이션**. 픽셀 아트보다 부드럽고 확장성이 좋으며, SVG 기반이라 투명 배경 및 고해상도 디스플레이에 최적화됩니다.
*   **캐릭터 명칭**: **"Draky" (드래키)** - 파일을 먹고 처리하는 작은 포털 생명체.
*   **상태별 애니메이션 (Framer Motion + Lottie)**:
    *   **Idle (대기)**: 미세하게 위아래로 떠 있는 느낌(Floating), 가끔 눈을 깜빡이거나 주변을 살핌.
    *   **Drag Over (근접)**: 입을 살짝 벌리며 파일 쪽으로 몸을 기울임. 주변에 은은한 **Indigo Glow** 효과 발생.
    *   **Eating (삼키기)**: 입을 크게 벌리고 파일이 중심부로 빨려 들어가는 **Black Hole/Suck-in** 효과.
    *   **Processing (처리 중)**: 몸이 회전하거나 위아래로 꿀렁이며 '소화' 중임을 표현. 100MB 이상 시 캐릭터 하단에 **Circular Progress Ring** 노출.
    *   **Success (성공)**: "꿀꺽" 하는 모션과 함께 반짝이는 이펙트(Sparkle), 만족스러운 표정.
    *   **Fail (실패)**: 파일을 퉤 뱉어내는 모션 또는 연기가 나는 효과와 함께 어지러운 표정 노출.

### 2. Component Architecture (컴포넌트 구조)

*   **Styling**: **Tailwind CSS + CSS Modules**. 오버레이는 정밀한 애니메이션 제어를 위해 모듈형 CSS를 혼용합니다.
*   **Component Tree**:
    *   `OverlayApp`: 최상단 투명 윈도우 래퍼.
        *   `CharacterContainer`: 드래그 앤 드롭 이벤트 핸들러 및 위치 관리.
            *   `DrakyVisual`: Lottie 애니메이션 엔진.
            *   `StatusIndicator`: 성공/실패 토스트 및 프로그레스 바.
    *   `DashboardApp`: 메인 설정/로그 패널.
        *   `Navigation`: 설정/로그/규칙 탭 전환.
        *   `RunLogList`: 워크플로 실행 이력 (Bento Box 스타일 카드 레이아웃).
        *   `RuleEditor`: 자연어 기반 룰 편집 및 JSON 뷰어.

### 3. Animation System (애니메이션 시스템)

*   **삼키기 효과 (The Swallow)**:
    *   `framer-motion`의 `layout` 프로퍼티와 `scale: 0`을 결합.
    *   파일 아이콘(시스템 제공)이 캐릭터 중심 좌표로 곡선을 그리며 이동(Bezier Curve)하면서 동시에 작아짐.
*   **진행률 표시**: 캐릭터 외곽을 감싸는 2px 두께의 **Gradient Progress Ring**.
*   **피드백 루프**: 
    *   성공 시: 캐릭터가 살짝 커졌다가(Scale Up) 제자리로 돌아옴.
    *   실패 시: 좌우로 흔들리는(Shake) 애니메이션.

### 4. Window Architecture (윈도우 아키텍처)

*   **Overlay Window (Transparency)**: 
    *   Tauri의 `ignore_cursor_events` 기능을 활용하여 캐릭터 이미지 외의 영역은 클릭-스루(Click-through) 처리.
    *   `always_on_top: true`, `decorations: false`.
*   **Main Panel Window**:
    *   별도의 윈도우 레이블(`main`)로 관리.
    *   오버레이에서 "Details" 클릭 시 `WebviewWindow.get_window('main').show()` 호출.
*   **Inter-window Comm**: Tauri의 `emit` / `listen` 이벤트를 통해 실시간 상태 공유.

### 5. Design Tokens & Theme (디자인 토큰)

*   **Typography**: **"Geist Sans"** 또는 **"Pretendard JP"** (현대적이고 깔끔한 산세리프). 숫자는 고정폭(Tabular Figures) 폰트 사용.
*   **Color Palette**:
    *   `Primary`: `#6366f1` (Indigo 500) - 메인 브랜드 컬러.
    *   `Success`: `#10b981` (Emerald 500) - 성공 상태.
    *   `Danger`: `#ef4444` (Red 500) - 실패 및 위험 설정.
    *   `Background`: `#0f172a` (Slate 900) - 다크 모드 기반의 패널 배경.
*   **Spacing**: 4px(Base) 단위의 시스템. 패널 내 카드는 `rounded-2xl` 적용으로 부드러운 인상 부여.

### Verification (검증 전략)
- **Tauri Dev**: `npm run tauri dev`를 통한 실시간 오버레이 투명도 및 드롭 이벤트 테스트.
- **Responsiveness**: 오버레이는 128px 고정이지만, 대형 파일 프로그레스 노출 시 레이아웃 확장성 확인.
- **Accessibility**: 오버레이 캐릭터에 `aria-label` 부여, 관리 패널은 키보드 네비게이션 지원.