# SPEC.md 리뷰 합의 문서 (Consensus Document)

**날짜**: 2026-03-04
**대상**: Draction + OpenClaw Bridge SPEC (v0.1)
**참여**: Planner (분석), Architect (아키텍처 검토), Critic (비평/필터링)
**Iteration**: 2 (최종 합의)

---

## 요약 매트릭스

| # | 항목 | 심각도 | 소속 문서 | Planner | Architect | Critic | 합의 |
|---|------|--------|-----------|---------|-----------|--------|------|
| 1 | API 엔드포인트 정의 | **CRITICAL** | SPEC | Critical | P2 | 유일한 Critical | **CRITICAL** |
| 2 | 이벤트 스키마 누락 필드 | HIGH | SPEC | High | 동의 | 동의 | **HIGH** |
| 3 | Rule 모델 우선순위/충돌 | MEDIUM | SPEC (v0.1 결정) | Missing | P0 | v0.1 불필요 | **MEDIUM** |
| 4 | Workflow 에러 처리 | HIGH | SPEC | High | 동의 | 동의 | **HIGH** |
| 5 | 프로세스 라이프사이클 | HIGH | SPEC + Design Doc | - | A-1 신규 | - | **HIGH** |
| 6 | Overlay 동작 상세 | MEDIUM | SPEC | Medium | 동의 | 동의 | **MEDIUM** |
| 7 | Undo 메커니즘 상세 | MEDIUM | SPEC | Medium | 동의 | 동의 | **MEDIUM** |
| 8 | 인증 플로우 상세 | LOW | SPEC | Medium | 동의 | 과도 | **LOW** |
| 9 | 기술 스택/플랫폼 | RECOMMEND | ADR | Critical | 동의 | ADR로 분리 | **ADR** |
| 10 | Workflow 상태 머신 | RECOMMEND | Design Doc | - | A-2 신규 | - | **Design Doc** |

---

## Part A: SPEC.md에 직접 추가할 항목 (8건)

---

### 1. [CRITICAL] API 엔드포인트 정의

**근거**: OpenClaw와 Draction이 독립 개발되려면 API 계약이 필수. 현재 SPEC에는 "로컬 HTTP + WebSocket"이라는 방식만 있고, 구체적 엔드포인트가 없음. 세 에이전트 모두 이 항목의 필요성에 동의.

**SPEC 3절 끝에 추가할 내용:**

```markdown
### API 엔드포인트 (v0.1)

Base: `http://127.0.0.1:{port}/api/v1`

#### Rules
| Method | Path | 설명 |
|--------|------|------|
| GET | `/rules` | 전체 룰 목록 |
| GET | `/rules/:id` | 룰 상세 |
| POST | `/rules` | 룰 생성 (body: Rule JSON) |
| PUT | `/rules/:id` | 룰 수정 |
| DELETE | `/rules/:id` | 룰 삭제 |
| PATCH | `/rules/:id/enabled` | 활성/비활성 토글 |

#### Workflows
| Method | Path | 설명 |
|--------|------|------|
| GET | `/workflows` | 전체 워크플로 목록 |
| GET | `/workflows/:id` | 워크플로 상세 |
| POST | `/workflows` | 워크플로 생성 |
| PUT | `/workflows/:id` | 워크플로 수정 |

#### Runs (로그)
| Method | Path | 설명 |
|--------|------|------|
| GET | `/runs` | 실행 이력 (쿼리: `?status=failed&limit=20`) |
| GET | `/runs/:id` | 실행 상세 + 노드별 로그 |

#### Events
| Method | Path | 설명 |
|--------|------|------|
| GET | `/events` | 최근 이벤트 목록 |

#### WebSocket
| Path | 설명 |
|------|------|
| `ws://127.0.0.1:{port}/ws` | 이벤트 스트림 구독 |

WS 메시지 포맷:
\```json
{ "channel": "events", "payload": { /* EVENT_INGESTED | RUN_* */ } }
\```

#### 공통 에러 응답
\```json
{
  "error": { "code": "RULE_NOT_FOUND", "message": "Rule rule_xxx does not exist" }
}
\```
HTTP 상태 코드: 400 (유효성), 401 (인증), 404 (미존재), 500 (내부 오류)
```

**수용 기준**: OpenClaw 개발자가 이 표만 보고 API 클라이언트를 구현할 수 있어야 함.

---

### 2. [HIGH] 이벤트 스키마 누락 필드

**근거**: 현재 EVENT_INGESTED에 파일 해시가 없어 중복 감지 불가. RUN_FAILED에 에러 상세가 없음.

**SPEC 4절 수정:**

```json
// EVENT_INGESTED.files[] 각 항목에 추가:
{
  "path": "/Users/me/Draction/Inbox/2026-03-04/a.mov",
  "name": "a.mov",
  "ext": "mov",
  "sizeBytes": 123456789,
  "mime": "video/quicktime",
  "sha256": "abcdef..."       // 추가: 중복 감지용
}
```

```json
// RUN_FAILED 스키마 (4절에 추가)
{
  "type": "RUN_FAILED",
  "runId": "run_...",
  "eventId": "evt_...",
  "ruleId": "rule_...",
  "workflowId": "wf_...",
  "failedNodeId": "n2",
  "error": {
    "code": "S3_UPLOAD_TIMEOUT",
    "message": "Connection timed out after 30s",
    "retryable": true
  },
  "partialArtifacts": []
}
```

**수용 기준**: 모든 이벤트 타입(INGESTED, RUN_STARTED, RUN_FINISHED, RUN_FAILED)에 완전한 JSON 예시가 존재.

---

### 3. [HIGH] Workflow 에러 처리 정책

**근거**: 노드 실패 시 동작이 정의되지 않음. v0.1이라도 실패 시나리오는 반드시 명시 필요.

**SPEC 6절 끝에 추가:**

```markdown
### 에러 처리 (v0.1)

- **기본 정책**: 노드 실패 시 워크플로 즉시 중단 (fail-fast)
- **재시도**: v0.1에서는 자동 재시도 없음. 수동 재실행만 지원 (`POST /runs/:id/retry`)
- **부분 결과**: 실패 전 생성된 artifact는 보존. `partialArtifacts`로 조회 가능
- **알림**: RUN_FAILED 이벤트를 WS로 전송. OpenClaw 연결 시 UI 토스트 표시

> v0.2 예정: 노드별 retry 정책, fallback 노드, 타임아웃 설정
```

**수용 기준**: "transcode 노드가 실패하면 어떻게 되는가?"에 SPEC만으로 답할 수 있어야 함.

---

### 4. [HIGH] 프로세스 라이프사이클 (Architect 신규 발견)

**근거**: Draction 프로세스의 시작/종료/충돌 복구가 정의되지 않음. 데스크톱 앱 특성상 필수.

**SPEC 2절 A항 끝에 추가:**

```markdown
### 프로세스 라이프사이클 (v0.1)

- **시작**: 로그인 시 자동 실행 (OS Login Item). 포트 충돌 시 기존 인스턴스에 위임
- **단일 인스턴스**: lock 파일(`~/Draction/.lock`)로 중복 실행 방지
- **종료**: 실행 중인 워크플로가 있으면 완료 대기 후 종료 (최대 30초, 이후 강제 종료)
- **충돌 복구**: 재시작 시 `runs` DB에서 status=running인 항목을 FAILED로 마킹
- **상태 파일**: `~/Draction/state.json` — 마지막 실행 시각, 포트 번호, PID 기록
```

**수용 기준**: Draction이 비정상 종료 후 재시작했을 때의 동작을 SPEC만으로 설명할 수 있어야 함.

---

### 5. [MEDIUM] Rule 우선순위와 충돌 처리 (v0.1 결정)

**근거**: Architect는 P0로 올렸으나, Critic의 지적대로 v0.1은 1-2개 룰로 시작. 다만 "의도적으로 직렬 실행"이라는 결정 자체를 SPEC에 명시해야 향후 혼란 방지.

**SPEC 5절 끝에 추가:**

```markdown
### 룰 실행 정책 (v0.1)

- **매칭 순서**: 룰 목록의 등록 순서대로 평가 (FIFO)
- **다중 매칭**: 하나의 파일에 여러 룰이 매칭되면 **첫 번째 매칭 룰만 실행** (first-match-wins)
- **동시성**: v0.1은 직렬 실행. 파일 1개 처리 완료 후 다음 파일 처리
- **우선순위 필드**: v0.1에서는 미지원. 등록 순서가 암묵적 우선순위

> v0.2 예정: `priority` 필드 도입, 다중 룰 동시 실행, 충돌 감지
```

**수용 기준**: "같은 파일에 2개 룰이 매칭되면?"이라는 질문에 SPEC만으로 답할 수 있어야 함.

---

### 6. [MEDIUM] Overlay 동작 상세

**근거**: 드롭 영역 크기, 다중 파일 드롭, 폴더 드롭 동작이 미정의.

**SPEC 1절 1항에 추가:**

```markdown
#### 드롭 상세 (v0.1)
- **드롭 영역**: 기본 128x128px 오버레이 아이콘. 위치는 사용자 드래그로 변경 가능
- **다중 파일**: 복수 파일 동시 드롭 가능. 각 파일마다 개별 EVENT_INGESTED 발생
- **폴더 드롭**: 폴더 자체를 하나의 단위로 이동. 내부 파일은 개별 이벤트 미발생 (v0.1)
- **드래그 오버 피드백**: 파일이 오버레이 위에 올라오면 시각적 하이라이트 (확대/발광)
- **진행 표시**: 대용량 파일(>100MB) 이동 시 프로그레스 표시
```

**수용 기준**: 프론트엔드 개발자가 Overlay 컴포넌트를 이 스펙만으로 구현 시작할 수 있어야 함.

---

### 7. [MEDIUM] Undo 메커니즘 상세

**근거**: "10초 내 되돌리기"만 있고, 구체적 구현 계약이 없음.

**SPEC 7절 Undo 항목 교체:**

```markdown
- **Undo**:
  - 드롭 후 10초 내 "되돌리기" 가능 (move의 경우 원위치 복원)
  - Undo 스택: 최근 5건 유지. 10초 경과 또는 해당 파일의 워크플로 실행 시작 시 Undo 불가
  - copy 모드의 경우: Inbox 사본 삭제
  - API: `POST /api/v1/events/:eventId/undo` (성공 시 원본 경로 반환)
  - 워크플로 실행 중 Undo 요청 시: 409 Conflict 응답
```

**수용 기준**: Undo 버튼 클릭 시 정확히 어떤 API를 호출하고 어떤 응답을 받는지 명확해야 함.

---

### 8. [LOW] 인증 플로우 상세

**근거**: "1회성 코드 또는 shared token"이라고만 되어 있음. v0.1 최소 수준으로 명확화.

**SPEC 3절 인증 부분 보완:**

```markdown
### 인증 플로우 (v0.1)

1. Draction 최초 실행 시 랜덤 토큰 생성 → `~/Draction/config.json`에 저장
2. OpenClaw 최초 연결 시:
   - Draction 트레이 아이콘에 "연결 요청" 알림 표시
   - 사용자가 승인하면 토큰을 OpenClaw에 전달 (1회)
   - OpenClaw는 토큰을 자체 설정에 저장
3. 이후 모든 요청: `Authorization: Bearer <token>` 헤더 포함
4. 토큰 재발급: Draction Settings에서 "토큰 초기화" → 기존 연결 해제

> 보안 범위: localhost 바인딩(127.0.0.1) + Bearer 토큰. 외부 네트워크 노출 없음.
```

**수용 기준**: 페어링 과정을 사용자에게 설명할 수 있을 정도로 명확해야 함.

---

## Part B: 별도 문서로 분리할 항목 (2건)

---

### 9. [ADR 필요] 기술 스택/플랫폼 선택

**근거**: Planner는 Critical로, Architect도 동의했으나, Critic의 지적이 타당 -- SPEC은 "무엇(WHAT)"을 정의하고, 기술 스택은 "어떻게(HOW)"에 해당. 별도 ADR(Architecture Decision Record)로 분리.

**SPEC 9절 끝에 참조만 추가:**

```markdown
> 기술 스택 선택(Electron vs Tauri, 언어 선택 등)은 별도 ADR 문서 참조.
> → `docs/adr/001-tech-stack.md` (작성 예정)
```

**ADR 문서에 포함할 내용 (별도 작성):**
- Electron vs Tauri vs Swift(macOS only) 비교
- Overlay 구현 방식이 프레임워크에 따라 다른 점 (Architect 지적 반영)
- 언어 선택: TypeScript vs Rust vs 혼합
- DB: SQLite 확정 여부

---

### 10. [Design Doc 필요] Workflow 상태 머신 (Architect 신규 발견)

**근거**: 워크플로 노드의 상태 전이(pending -> running -> success/failed)가 정의되지 않음. SPEC에는 너무 상세하므로 Design Doc으로 분리.

**SPEC 6절 끝에 참조만 추가:**

```markdown
> 워크플로 실행의 상태 머신(노드별 상태 전이, 재시도 로직 등)은 별도 설계 문서 참조.
> → `docs/design/workflow-state-machine.md` (작성 예정)
```

**Design Doc에 포함할 내용 (별도 작성):**
- 노드 상태: `pending` -> `running` -> `success` | `failed` | `skipped`
- 워크플로 상태: `queued` -> `running` -> `completed` | `failed` | `cancelled`
- FS watch 경계 (Architect A-3): Inbox 디렉토리 감시 범위와 이벤트 디바운싱

---

## 합의 요약

### 채택된 관점 정리

| 쟁점 | 합의 결과 | 근거 |
|------|-----------|------|
| API 엔드포인트 심각도 | **CRITICAL** (Critic 의견 채택) | 두 앱 간 유일한 계약 |
| 기술 스택 위치 | **ADR로 분리** (Critic 의견 채택) | SPEC은 WHAT, 스택은 HOW |
| Rule 충돌/동시성 | **MEDIUM + v0.1 결정 명시** (절충) | v0.1 범위에선 과도하나 결정 자체는 기록 필요 |
| 프로세스 라이프사이클 | **HIGH, SPEC에 포함** (Architect 의견 채택) | 데스크톱 앱 필수 요소 |
| Workflow 상태 머신 | **Design Doc으로 분리** (절충) | SPEC에는 과도, 그러나 설계 필수 |
| 항목 수 제한 | **10건** (Critic 의견 채택) | 22건 -> 10건으로 압축 |

### 반영하지 않은 항목 (이유 포함)

- **FS Watch 경계 (A-3)**: Design Doc #10에 통합. SPEC 수준이 아닌 구현 상세.
- **플러그인 아키텍처**: v0.1 범위 초과. v0.2+ 논의 사항.
- **다국어/로컬라이제이션**: v0.1 범위 초과.
- **성능 요구사항(비기능)**: v0.1에서는 "동작하면 됨" 수준. 벤치마크는 v0.2.
- **테스트 전략**: SPEC이 아닌 개발 프로세스 문서에 해당.

---

## 실행 가이드

**즉시 실행 (SPEC.md 편집):**
1. 항목 #1 (API 엔드포인트) -- 가장 높은 우선순위, OpenClaw 개발 차단 해소
2. 항목 #2, #3 (이벤트 스키마, 에러 처리) -- 구현 시작 전 필수
3. 항목 #4 (프로세스 라이프사이클) -- 데스크톱 앱 기본 동작 정의
4. 항목 #5-8 (나머지 SPEC 항목) -- 구현 중 병행 추가 가능

**후속 문서 작성:**
5. `docs/adr/001-tech-stack.md` -- 기술 스택 결정 후 구현 시작
6. `docs/design/workflow-state-machine.md` -- Workflow 엔진 구현 전 작성
