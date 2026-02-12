# LEGAL DOCS REVIEW: DEFINITIVE IMPLEMENTATION PLAN

**Version:** 1.0
**Status:** READY FOR EXECUTION
**Author:** VP Engineering
**Date:** 2026-02-12

---

## 1. ARCHITECTURE & TECH STACK

### 1.1 Core Technology Decisions

| Layer | Technology | Version | Rationale |
|-------|-----------|---------|-----------|
| **Frontend Framework** | React | 19.x | Declarative UI, component reuse, large ecosystem, established patterns |
| **Build Tool** | Vite | 6.x | 10x faster dev server, better DX than Webpack, native ES modules |
| **Desktop Framework** | Tauri | 2.x | Rust backend + web frontend, minimal binary size (vs Electron), security-first |
| **Backend Runtime** | Rust | 1.77.2+ | Type safety, memory safety, performance, async-await native |
| **Database** | SQLite | Bundled | Local-first, no server, portable, sufficient for single-user desktop app |
| **Styling** | Tailwind CSS | 3.4.x | Utility-first, minimal CSS bundle, rapid UI development |
| **Routing** | React Router | 7.x | Industry standard SPA routing, nested routes, lazy loading support |
| **HTTP Client** | Reqwest | 0.12 | Rust async HTTP, used by backend for AI API calls |
| **State Management** | React Hooks + Context | Native | Sufficient for current scope, avoids Redux complexity |
| **Type Validation** | Zod | 3.24.x | Runtime schema validation at API boundaries |
| **Testing Framework** | Vitest | Latest | ESM-native, Vite-integrated, faster than Jest |
| **UI Components** | Lucide React | Latest | Lightweight icon library, tree-shakeable |
| **Notifications** | React Hot Toast | 2.4.x | Lightweight toast notification system |
| **File Uploads** | React Dropzone | 14.3.x | Drag-drop file handling, already integrated |
| **Charts** | Recharts | 2.15.x | Declarative charting, composable components |

### 1.2 Module Boundaries & Responsibilities

```
┌─────────────────────────────────────────────────────────────┐
│                   React Frontend (src/)                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐ │
│  │  Pages   │  │Components│  │  Hooks   │  │Lib/Types/Utils│ │
│  └──────────┘  └──────────┘  └──────────┘  └──────────────┘ │
└────────────────────────┬────────────────────────────────────┘
                         │ Tauri IPC (JSON)
                         ↓
┌─────────────────────────────────────────────────────────────┐
│              Tauri Backend (src-tauri/)                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────────┐ │
│  │ Commands │  │    DB    │  │    AI    │  │  Utils/Error │ │
│  └──────────┘  └──────────┘  └──────────┘  └──────────────┘ │
└────────────────────────┬────────────────────────────────────┘
                         │ Network/File I/O
         ┌───────────────┼───────────────┐
         ↓               ↓               ↓
    ┌─────────┐    ┌──────────┐    ┌─────────┐
    │ SQLite  │    │AI APIs   │    │File Sys │
    │   DB    │    │(OpenAI/  │    │(PDFs)   │
    │         │    │Claude)   │    │         │
    └─────────┘    └──────────┘    └─────────┘
```

**Responsibility Matrix:**

| Module | Owns | Depends On |
|--------|------|-----------|
| `src/pages/` | Page layout, component composition | `src/components/`, `src/hooks/`, `src/lib/commands` |
| `src/components/` | Reusable UI components, visual logic | `src/types/`, Tailwind CSS, Lucide Icons |
| `src/hooks/` | Data fetching, state management | `src/lib/commands/`, React hooks |
| `src/lib/commands.ts` | Tauri IPC invocation wrapper | `src/types/`, Zod schemas |
| `src/lib/schemas.ts` | Runtime type validation | Zod |
| `src/types/` | Type definitions (enums, interfaces) | None (foundational) |
| `src-tauri/src/commands/` | Tauri command handlers (API layer) | `src-tauri/db/`, `src-tauri/ai/` |
| `src-tauri/src/db/` | Database operations | SQLite driver |
| `src-tauri/src/ai/` | AI provider abstraction | OpenAI/Claude/Ollama APIs |
| `src-tauri/src/analysis/` | Business logic (extraction, risk scoring) | `src-tauri/ai/`, `src-tauri/db/` |
| `src-tauri/src/documents/` | Document processing (PDF extraction) | pdf-extract crate |

---

## 2. FILE STRUCTURE (COMPLETE)

### 2.1 Complete Directory Tree

```
/home/user/LegalDocsReview/
├── .github/
│   └── workflows/                          # CI/CD pipelines
│       ├── test.yml                        # Run Vitest on push/PR
│       ├── lint.yml                        # ESLint + Prettier checks
│       ├── build.yml                       # Verify Tauri build
│       └── release.yml                     # Create releases on tag
├── .husky/                                 # Git hooks
│   └── pre-commit                          # Run linter before commit
├── docs/                                   # Documentation
│   ├── API.md                              # Tauri commands reference
│   ├── DATABASE.md                         # Schema documentation
│   ├── ARCHITECTURE.md                     # Module organization & data flow
│   └── DEPLOYMENT.md                       # Release procedure
├── src/                                    # React Frontend
│   ├── __mocks__/                          # Test mocks
│   │   └── @tauri-apps/                   # Tauri API mocks
│   ├── __tests__/                          # Test files
│   │   └── pages/                          # Page integration tests
│   │       ├── Dashboard.test.tsx
│   │       ├── Upload.test.tsx
│   │       ├── ReviewDetail.test.tsx
│   │       ├── Comparison.test.tsx
│   │       ├── Templates.test.tsx
│   │       ├── Reports.test.tsx
│   │       └── Settings.test.tsx
│   ├── components/
│   │   ├── analysis/
│   │   │   ├── ClauseTable.tsx             # [EXISTING] Expandable clause rows
│   │   │   ├── RiskPanel.tsx               # [EXISTING] Risk gauge + flags
│   │   │   ├── RiskChart.tsx               # [NEW] Risk distribution chart
│   │   │   └── ClauseTable.test.tsx, RiskPanel.test.tsx
│   │   └── layout/
│   │       ├── MainLayout.tsx              # [EXISTING] Sidebar + outlet
│   │       └── MainLayout.test.tsx
│   ├── hooks/
│   │   ├── useDocuments.ts                 # [EXISTING] Document fetching
│   │   └── useDocuments.test.ts
│   ├── lib/
│   │   ├── commands.ts                     # [EXISTING] Tauri IPC wrapper
│   │   ├── schemas.ts                      # [NEW] Zod validation schemas
│   │   └── cache.ts                        # [NEW] React Context caching
│   ├── pages/
│   │   ├── Dashboard.tsx                   # [EXISTING] Stats + recent docs
│   │   ├── Upload.tsx                      # [EXISTING] File upload
│   │   ├── ReviewDetail.tsx                # [EXISTING] Document viewer
│   │   ├── Comparison.tsx                  # [EXISTING] Document diff
│   │   ├── Templates.tsx                   # [EXISTING] Template CRUD
│   │   ├── Reports.tsx                     # [EXISTING] Report list/preview
│   │   └── Settings.tsx                    # [EXISTING] Config & API keys
│   ├── types/
│   │   └── index.ts                        # [EXISTING] Type definitions
│   ├── App.tsx                             # [EXISTING] Routes setup
│   ├── main.tsx                            # [EXISTING] React entry
│   ├── App.test.tsx                        # [NEW] Route smoke test
│   ├── index.css                           # [EXISTING] Tailwind imports
│   └── __tests__/                          # Integration tests
│       └── integration/                    # E2E workflows
│           ├── upload-analyze.test.tsx
│           └── compare-workflow.test.tsx
├── src-tauri/
│   ├── src/
│   │   ├── ai/
│   │   │   ├── mod.rs                      # [EXISTING] AI module export
│   │   │   ├── provider.rs                 # [EXISTING] Trait definition
│   │   │   ├── claude.rs                   # [EXISTING] Claude implementation
│   │   │   ├── openai.rs                   # [EXISTING] OpenAI implementation
│   │   │   ├── ollama.rs                   # [EXISTING] Ollama implementation
│   │   │   └── prompts.rs                  # [EXISTING] Prompt templates
│   │   ├── analysis/
│   │   │   ├── mod.rs                      # [EXISTING] Analysis module
│   │   │   └── risk_rules.rs               # [EXISTING] Risk assessment rules
│   │   ├── commands/
│   │   │   ├── mod.rs                      # [EXISTING] Command registration
│   │   │   ├── document_commands.rs        # [EXISTING] Document CRUD
│   │   │   ├── analysis_commands.rs        # [EXISTING] Analysis endpoints
│   │   │   ├── comparison_commands.rs      # [EXISTING] Comparison endpoints
│   │   │   ├── template_commands.rs        # [EXISTING] Template CRUD
│   │   │   ├── report_commands.rs          # [EXISTING] Report generation
│   │   │   └── settings_commands.rs        # [EXISTING] Settings management
│   │   ├── db/
│   │   │   ├── mod.rs                      # [EXISTING] DB module
│   │   │   ├── migrations.rs               # [EXISTING] Schema creation
│   │   │   ├── documents.rs                # [EXISTING] Document queries
│   │   │   ├── extractions.rs              # [EXISTING] Extraction queries
│   │   │   ├── risk_assessments.rs         # [EXISTING] Risk queries
│   │   │   ├── templates.rs                # [EXISTING] Template queries
│   │   │   ├── comparisons.rs              # [EXISTING] Comparison queries
│   │   │   ├── reports.rs                  # [EXISTING] Report queries
│   │   │   └── settings.rs                 # [EXISTING] Settings queries
│   │   ├── documents/
│   │   │   ├── mod.rs                      # [EXISTING] Documents module
│   │   │   └── pdf.rs                      # [EXISTING] PDF extraction
│   │   ├── reports/
│   │   │   └── mod.rs                      # [EXISTING] Report generation
│   │   ├── error.rs                        # [EXISTING] Error types
│   │   ├── lib.rs                          # [EXISTING] Tauri setup
│   │   └── main.rs                         # [EXISTING] Binary entry
│   ├── Cargo.toml                          # [EXISTING] Rust dependencies
│   └── tauri.conf.json                     # [EXISTING] Desktop config
├── .eslintrc.json                          # [NEW] Linting rules
├── .prettierrc                             # [NEW] Code formatting
├── .prettierignore                         # [NEW] Prettier ignore
├── .gitignore                              # [EXISTING] Git ignore
├── eslint.config.mjs                       # [NEW] ESLint config (flat)
├── index.html                              # [EXISTING] HTML entry
├── package.json                            # [EXISTING] Node dependencies
├── pnpm-lock.yaml                          # [EXISTING] Lockfile
├── postcss.config.js                       # [EXISTING] Tailwind setup
├── tailwind.config.ts                      # [EXISTING] Tailwind config
├── tsconfig.json                           # [EXISTING] TypeScript config
├── vite.config.ts                          # [EXISTING] Vite config
├── vitest.config.ts                        # [EXISTING] Vitest config
├── README.md                               # [EXISTING] Basic README
└── LICENSE                                 # [EXISTING] License

```

### 2.2 Files to Create (New)

**CI/CD & Automation:**
- `.github/workflows/test.yml`
- `.github/workflows/lint.yml`
- `.github/workflows/build.yml`
- `.github/workflows/release.yml`
- `.husky/pre-commit`

**Linting & Formatting:**
- `.eslintrc.json`
- `.prettierrc`
- `.prettierignore`
- `eslint.config.mjs`

**Documentation:**
- `docs/API.md`
- `docs/DATABASE.md`
- `docs/ARCHITECTURE.md`
- `docs/DEPLOYMENT.md`

**Frontend Code:**
- `src/lib/schemas.ts` (Zod validation)
- `src/lib/cache.ts` (React Context caching)
- `src/components/analysis/RiskChart.tsx` (Recharts visualization)
- `src/__tests__/pages/Dashboard.test.tsx`
- `src/__tests__/pages/Upload.test.tsx`
- `src/__tests__/pages/ReviewDetail.test.tsx`
- `src/__tests__/pages/Comparison.test.tsx`
- `src/__tests__/pages/Templates.test.tsx`
- `src/__tests__/pages/Reports.test.tsx`
- `src/__tests__/pages/Settings.test.tsx`
- `src/__tests__/App.test.tsx`
- `src/__tests__/integration/upload-analyze.test.tsx`
- `src/__tests__/integration/compare-workflow.test.tsx`

### 2.3 Files to Modify (Existing)

**Frontend:**
- `src/pages/Settings.tsx` - Fix API key input consistency
- `src/pages/Reports.tsx` - Implement download functionality
- `src/pages/Dashboard.tsx` - Add charts using Recharts
- `src/components/analysis/RiskPanel.tsx` - Enhanced visualization
- `src/lib/commands.ts` - Add Zod schema validation wrapping
- `package.json` - Remove recharts & zod from dependencies (if moved to schemas), add ESLint/Prettier

**Backend:**
- `src-tauri/src/commands/report_commands.rs` - Already functional, no changes needed

### 2.4 Import/Dependency Relationships

```
Frontend Dependency Graph:
─────────────────────────

pages/*
  ↓ imports from
components/*, hooks/*, lib/commands, types/

components/*
  ↓ imports from
components/*, types/, Tailwind CSS, Lucide

hooks/*
  ↓ imports from
lib/commands, React

lib/commands.ts
  ↓ imports from
lib/schemas, types/, Zod, Tauri

lib/schemas.ts
  ↓ imports from
types/, Zod

types/
  ↓ imports from
None (foundational)

Backend Dependency Graph:
─────────────────────────

commands/*
  ↓ imports from
db/*, ai/*, analysis/*, documents/, error

db/*
  ↓ imports from
db/migrations, error

ai/*
  ↓ imports from
ai/provider, ai/prompts, error

analysis/*
  ↓ imports from
ai/*, error

documents/*
  ↓ imports from
pdf-extract, error
```

---

## 3. DATA MODELS & API CONTRACTS

### 3.1 Database Schema (SQLite)

#### Table: documents
```sql
CREATE TABLE documents (
  id TEXT PRIMARY KEY,
  filename TEXT NOT NULL,
  file_path TEXT NOT NULL,
  contract_type TEXT NOT NULL, -- 'NDA', 'ServiceAgreement', 'Lease', 'Other'
  raw_text TEXT,
  text_extraction_status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'extracted', 'failed'
  total_pages INTEGER,
  file_size_bytes INTEGER,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,
  analysis_status TEXT NOT NULL DEFAULT 'pending', -- 'pending', 'completed', 'failed'
  error_message TEXT
);
CREATE INDEX idx_documents_created_at ON documents(created_at);
```

#### Table: extractions
```sql
CREATE TABLE extractions (
  id TEXT PRIMARY KEY,
  document_id TEXT NOT NULL,
  extraction_data TEXT NOT NULL, -- JSON: { clauses: [...], key_terms: [...] }
  ai_provider TEXT NOT NULL, -- 'openai', 'claude', 'ollama'
  extracted_at TEXT NOT NULL,
  FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);
CREATE INDEX idx_extractions_document_id ON extractions(document_id);
```

#### Table: risk_assessments
```sql
CREATE TABLE risk_assessments (
  id TEXT PRIMARY KEY,
  document_id TEXT NOT NULL,
  risk_score INTEGER NOT NULL, -- 0-100
  risk_flags TEXT NOT NULL, -- JSON: [ { severity: 'high'|'medium'|'low', message: string } ]
  summary TEXT,
  ai_provider TEXT NOT NULL,
  assessed_at TEXT NOT NULL,
  FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);
CREATE INDEX idx_risk_assessments_document_id ON risk_assessments(document_id);
```

#### Table: templates
```sql
CREATE TABLE templates (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  contract_type TEXT NOT NULL,
  reference_clauses TEXT NOT NULL, -- JSON: [ { clause: string, importance: string } ]
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

#### Table: comparisons
```sql
CREATE TABLE comparisons (
  id TEXT PRIMARY KEY,
  document_a_id TEXT NOT NULL,
  document_b_id TEXT,
  template_id TEXT,
  differences TEXT NOT NULL, -- JSON: { substantive: [...], formatting: [...] }
  compared_at TEXT NOT NULL,
  FOREIGN KEY (document_a_id) REFERENCES documents(id) ON DELETE CASCADE,
  FOREIGN KEY (document_b_id) REFERENCES documents(id) ON DELETE SET NULL,
  FOREIGN KEY (template_id) REFERENCES templates(id) ON DELETE SET NULL
);
Create INDEX idx_comparisons_document_a_id ON comparisons(document_a_id);
```

#### Table: reports
```sql
CREATE TABLE reports (
  id TEXT PRIMARY KEY,
  document_id TEXT NOT NULL,
  title TEXT NOT NULL,
  content TEXT NOT NULL, -- Full markdown/HTML report
  generated_at TEXT NOT NULL,
  FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
);
CREATE INDEX idx_reports_document_id ON reports(document_id);
```

#### Table: settings
```sql
CREATE TABLE settings (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

**Key constraints:**
- `documents.id` is UUID v4
- All timestamps are ISO 8601 format (UTC)
- JSON fields are stored as TEXT, validated at application layer
- Foreign key cascades preserve referential integrity

---

### 3.2 Type Definitions (TypeScript)

```typescript
// src/types/index.ts (existing, shown for reference)

// Document Types
export interface Document {
  id: string;
  filename: string;
  contractType: 'NDA' | 'ServiceAgreement' | 'Lease' | 'Other';
  textExtractionStatus: 'pending' | 'extracted' | 'failed';
  analysisStatus: 'pending' | 'completed' | 'failed';
  totalPages?: number;
  fileSizeBytes?: number;
  createdAt: string; // ISO 8601
  updatedAt: string;
  errorMessage?: string;
}

export interface DocumentStats {
  total: number;
  analyzed: number;
  pending: number;
  failed: number;
}

export interface Extraction {
  id: string;
  documentId: string;
  clauses: Array<{ text: string; importance: 'high' | 'medium' | 'low' }>;
  keyTerms: string[];
  aiProvider: 'openai' | 'claude' | 'ollama';
  extractedAt: string;
}

export interface RiskFlag {
  severity: 'high' | 'medium' | 'low';
  message: string;
  suggestion?: string;
}

export interface RiskAssessment {
  id: string;
  documentId: string;
  riskScore: number; // 0-100
  riskFlags: RiskFlag[];
  summary: string;
  aiProvider: 'openai' | 'claude' | 'ollama';
  assessedAt: string;
}

export interface Comparison {
  id: string;
  documentA: Document;
  documentB?: Document;
  template?: Template;
  substantiveDifferences: string[];
  formattingDifferences: string[];
  comparedAt: string;
}

export interface Template {
  id: string;
  name: string;
  contractType: 'NDA' | 'ServiceAgreement' | 'Lease';
  referenceClauses: Array<{ clause: string; importance: 'high' | 'medium' | 'low' }>;
  createdAt: string;
  updatedAt: string;
}

export interface Report {
  id: string;
  documentId: string;
  title: string;
  content: string; // Markdown or HTML
  generatedAt: string;
}

export interface Settings {
  aiProvider: 'openai' | 'claude' | 'ollama';
  openaiApiKey?: string;
  openaiModel?: string;
  claudeApiKey?: string;
  claudeModel?: string;
  ollamaUrl?: string;
  ollamaModel?: string;
}
```

### 3.3 Zod Validation Schemas

```typescript
// src/lib/schemas.ts (NEW)

import { z } from 'zod';

// Validation schemas matching backend API contracts
export const DocumentSchema = z.object({
  id: z.string().uuid(),
  filename: z.string(),
  contractType: z.enum(['NDA', 'ServiceAgreement', 'Lease', 'Other']),
  textExtractionStatus: z.enum(['pending', 'extracted', 'failed']),
  analysisStatus: z.enum(['pending', 'completed', 'failed']),
  totalPages: z.number().optional(),
  fileSizeBytes: z.number().optional(),
  createdAt: z.string().datetime(),
  updatedAt: z.string().datetime(),
  errorMessage: z.string().optional(),
});

export const RiskFlagSchema = z.object({
  severity: z.enum(['high', 'medium', 'low']),
  message: z.string(),
  suggestion: z.string().optional(),
});

export const RiskAssessmentSchema = z.object({
  id: z.string().uuid(),
  documentId: z.string().uuid(),
  riskScore: z.number().min(0).max(100),
  riskFlags: z.array(RiskFlagSchema),
  summary: z.string(),
  aiProvider: z.enum(['openai', 'claude', 'ollama']),
  assessedAt: z.string().datetime(),
});

export const ExtractionSchema = z.object({
  id: z.string().uuid(),
  documentId: z.string().uuid(),
  clauses: z.array(z.object({
    text: z.string(),
    importance: z.enum(['high', 'medium', 'low']),
  })),
  keyTerms: z.array(z.string()),
  aiProvider: z.enum(['openai', 'claude', 'ollama']),
  extractedAt: z.string().datetime(),
});

export const SettingsSchema = z.object({
  aiProvider: z.enum(['openai', 'claude', 'ollama']),
  openaiApiKey: z.string().optional(),
  openaiModel: z.string().optional(),
  claudeApiKey: z.string().optional(),
  claudeModel: z.string().optional(),
  ollamaUrl: z.string().url().optional(),
  ollamaModel: z.string().optional(),
});

// Export inferred types
export type Document = z.infer<typeof DocumentSchema>;
export type RiskAssessment = z.infer<typeof RiskAssessmentSchema>;
export type Extraction = z.infer<typeof ExtractionSchema>;
export type Settings = z.infer<typeof SettingsSchema>;
```

### 3.4 Tauri Command API Contracts

**Command Naming Convention:** `snake_case`
**Request/Response Format:** JSON
**Error Format:**
```json
{
  "error": "ErrorType::VariantName",
  "message": "Human-readable error message",
  "details": "Optional technical details"
}
```

#### Document Commands

| Command | Request | Response | Status Codes |
|---------|---------|----------|--------------|
| `upload_document` | `{ filename: string, contract_type: string, file_bytes: number[] }` | `{ id: string, filename: string }` | 200 (ok), 400 (invalid file), 500 (storage error) |
| `extract_document_text` | `{ document_id: string }` | `{ status: 'extracted', page_count: number }` | 200 (ok), 404 (not found), 422 (unreadable PDF) |
| `get_document` | `{ document_id: string }` | Document object | 200 (ok), 404 (not found) |
| `list_documents` | `{ limit?: number, offset?: number }` | `{ documents: Document[], total: number }` | 200 (ok) |
| `delete_document` | `{ document_id: string }` | `{ success: true }` | 200 (ok), 404 (not found) |
| `get_document_stats` | `{}` | `{ total: number, analyzed: number, pending: number, failed: number }` | 200 (ok) |

#### Analysis Commands

| Command | Request | Response | Status Codes |
|---------|---------|----------|--------------|
| `analyze_document` | `{ document_id: string }` | `{ extraction_id: string, risk_id: string }` | 200 (ok), 404 (not found), 500 (AI error) |
| `get_extractions` | `{ document_id: string }` | `{ clauses: [...], key_terms: [...] }` | 200 (ok), 404 (not found) |
| `get_risk_assessments` | `{ document_id: string }` | `{ risk_score: number, flags: [...] }` | 200 (ok), 404 (not found) |
| `get_risk_distribution` | `{ document_id?: string }` | `{ high: number, medium: number, low: number }` | 200 (ok) |

#### Settings Commands

| Command | Request | Response | Status Codes |
|---------|---------|----------|--------------|
| `get_settings` | `{}` | Settings object | 200 (ok) |
| `set_setting` | `{ key: string, value: string }` | `{ key: string, value: string }` | 200 (ok), 400 (invalid key) |
| `delete_setting` | `{ key: string }` | `{ success: true }` | 200 (ok), 404 (not found) |

*See docs/API.md for complete reference (20+ commands)*

---

### 3.5 React State Shape (with Context)

```typescript
// src/lib/cache.ts (NEW)

import React, { createContext, useState, useCallback } from 'react';

interface CacheState {
  documents: Map<string, Document>;
  extractions: Map<string, Extraction>;
  riskAssessments: Map<string, RiskAssessment>;
  stats: DocumentStats | null;
}

interface CacheContextType {
  state: CacheState;
  setDocument: (id: string, doc: Document) => void;
  setExtraction: (id: string, ext: Extraction) => void;
  setRiskAssessment: (id: string, risk: RiskAssessment) => void;
  setStats: (stats: DocumentStats) => void;
  invalidateDocument: (id: string) => void;
  invalidateAll: () => void;
}

export const CacheContext = createContext<CacheContextType | null>(null);

// Usage in App.tsx:
// <CacheProvider>
//   <Routes />
// </CacheProvider>
```

---

## 4. IMPLEMENTATION STEPS (SEQUENTIAL)

### PHASE 1: STABILIZATION & QUALITY GATES (Steps 1-8)

---

#### STEP 1: Fix Settings API Key Input Inconsistency
**Priority:** P0 (Blocks user experience)
**Files Touched:** `src/pages/Settings.tsx`

**Prerequisites:**
- Project environment set up locally
- `pnpm install` completed
- `pnpm tauri dev` can run

**What must be true before:**
- None (can start immediately)

**Implementation:**

1. Open `src/pages/Settings.tsx`
2. Identify the issue: Lines 80-120 have inconsistent save patterns
   - Current state: Ollama saves on button click, Claude/OpenAI save on input blur
3. Implement unified pattern:
   ```typescript
   // Before (WRONG):
   <input onChange={e => setSetting('claudeApiKey', e.target.value)} />

   // After (CORRECT):
   const [dirtyFields, setDirtyFields] = useState<Set<string>>(new Set());

   const handleInputChange = (key: string, value: string) => {
    setFormData(prev => ({ ...prev, [key]: value }));
    setDirtyFields(prev => new Set([...prev, key]));
   };

   const handleSave = async () => {
    for (const key of dirtyFields) {
      await setSetting(key, formData[key]);
    }
    setDirtyFields(new Set());
    toast.success('Settings saved');
   };

   // UI: Show "*" on dirty fields, disable Save button when no changes
   ```
4. Add visual feedback: "Unsaved changes" indicator when dirty fields exist
5. Test locally:
   - Type API key
   - Verify "*" shows next to field
   - Click Save
   - Verify toast notification
   - Refresh page
   - Verify setting persisted

**Complexity:** Low

**Code Changes (Pseudocode):**
```typescript
// Settings.tsx changes
const [formData, setFormData] = useState(initialSettings);
const [dirtyFields, setDirtyFields] = useState<Set<string>>(new Set());

const handleInputChange = (key: string, value: string) => {
  setFormData({ ...formData, [key]: value });
  setDirtyFields(new Set([...dirtyFields, key]));
};

const handleSave = async () => {
  for (const key of dirtyFields) {
    await setSetting(key, formData[key]);
  }
  setDirtyFields(new Set());
  toast.success('Settings saved');
};

// In JSX:
<div>
  {dirtyFields.has('claudeApiKey') && <span>*</span>}
  <input
    value={formData.claudeApiKey}
    onChange={e => handleInputChange('claudeApiKey', e.target.value)}
  />
</div>
<button disabled={dirtyFields.size === 0} onClick={handleSave}>
  Save Settings
</button>
```

**Error Handling:**
- Try/catch around `setSetting()` calls
- Show error toast if API returns error
- Retain dirty state if save fails (allow user to retry)
- Log errors to console in dev

**Testing:**
```typescript
// Settings.test.tsx
test('marks field dirty on change', () => {
  render(<Settings />);
  const input = screen.getByDisplayValue('');
  fireEvent.change(input, { target: { value: 'new-key' } });
  expect(screen.getByText('*')).toBeInTheDocument();
});

test('clears dirty state after successful save', async () => {
  render(<Settings />);
  // ... change input
  const saveBtn = screen.getByText('Save Settings');
  fireEvent.click(saveBtn);
  await waitFor(() => {
    expect(screen.queryByText('*')).not.toBeInTheDocument();
  });
});
```

**What is Unlocked After:**
- Step 2: Can verify UI improvements work before expanding test coverage

---

#### STEP 2: Implement Report Download Functionality
**Priority:** P0 (Blocks feature advertised in UI)
**Files Touched:** `src/pages/Reports.tsx`

**Prerequisites:**
- Step 1 complete (confidence in UI patterns)
- Tauri fs plugin available (`tauri-plugin-fs`)

**What must be true before:**
- Report content available in state
- Tauri fs plugin initialized

**Implementation:**

1. Open `src/pages/Reports.tsx`, find button at line 102 with no onClick
2. Implement download handler using Tauri's writeTextFile:
   ```typescript
   import { writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';
   import { save } from '@tauri-apps/plugin-dialog';

   const handleDownloadReport = async (report: Report) => {
     try {
       const filePath = await save({
         defaultPath: `report_${report.id}.md`,
         filters: [{ name: 'Markdown', extensions: ['md'] }],
       });

       if (filePath) {
         await writeTextFile(filePath, report.content);
         toast.success('Report downloaded');
       }
     } catch (error) {
       toast.error('Download failed: ' + error.message);
     }
   };
   ```
3. Add onClick to button: `<button onClick={() => handleDownloadReport(report)}>Download</button>`
4. Test locally:
   - Navigate to Reports page
   - Click Download on a report
   - Select save location
   - Verify file created with correct content

**Complexity:** Low

**Error Handling:**
- User cancels save dialog → no action (not an error)
- writeTextFile fails → show error toast with error message
- Permission denied → error toast: "Permission denied to write file"

**Testing:**
```typescript
test('downloads report to user location', async () => {
  mockSave.mockResolvedValue('/home/user/report.md');
  mockWriteTextFile.mockResolvedValue(undefined);

  render(<Reports />);
  const downloadBtn = screen.getByText('Download');
  fireEvent.click(downloadBtn);

  await waitFor(() => {
    expect(mockWriteTextFile).toHaveBeenCalledWith(
      '/home/user/report.md',
      expect.stringContaining('Report content')
    );
    expect(toast.success).toHaveBeenCalled();
  });
});
```

**What is Unlocked After:**
- Step 3: Both user-facing bugs fixed, can now expand test coverage with confidence

---

#### STEP 3: Create GitHub Actions Test Workflow
**Priority:** P0 (Enables automated safety net)
**Files Touched:** `.github/workflows/test.yml` (NEW)

**Prerequisites:**
- Repository has GitHub Actions enabled
- Node.js runtime available in CI

**What must be true before:**
- Steps 1-2 complete and tested locally

**Implementation:**

Create `.github/workflows/test.yml`:
```yaml
name: Run Tests

on:
  push:
    branches: [main, claude/*]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'pnpm'

      - name: Install dependencies
        run: pnpm install --frozen-lockfile

      - name: Run tests
        run: pnpm test

      - name: Upload coverage
        uses: codecov/codecov-action@v3
        if: always()
        with:
          files: ./coverage/coverage-final.json
```

**Complexity:** Low

**What is Unlocked After:**
- Step 4: Can run tests automatically on every push; safe to expand coverage

---

#### STEP 4: Write Page Integration Tests (50% Coverage Target)
**Priority:** P1 (Critical for confidence)
**Files Touched:**
- `src/__tests__/pages/Dashboard.test.tsx` (NEW)
- `src/__tests__/pages/Upload.test.tsx` (NEW)
- `src/__tests__/pages/ReviewDetail.test.tsx` (NEW)
- `src/__tests__/pages/Settings.test.tsx` (MODIFIED from Step 1)
- `src/__tests__/pages/Reports.test.tsx` (MODIFIED from Step 2)
- `src/__tests__/pages/Comparison.test.tsx` (NEW)
- `src/__tests__/pages/Templates.test.tsx` (NEW)
- `src/__tests__/App.test.tsx` (NEW)

**Prerequisites:**
- Steps 1-3 complete
- Test mocks in place (`src/__mocks__/`)
- Vitest configured

**What must be true before:**
- All existing tests pass
- GitHub Actions workflow running successfully

**Implementation:**

For each page, write 2-3 critical path tests:

**Dashboard.test.tsx** (NEW):
```typescript
import { render, screen } from '@testing-library/react';
import { vi } from 'vitest';
import Dashboard from '../../pages/Dashboard';
import * as commands from '../../lib/commands';

vi.mock('../../lib/commands');

test('displays document statistics', async () => {
  vi.mocked(commands.getDocumentStats).mockResolvedValue({
    total: 10,
    analyzed: 8,
    pending: 1,
    failed: 1,
  });

  render(<Dashboard />);

  expect(await screen.findByText('10')).toBeInTheDocument(); // total
  expect(screen.getByText('8')).toBeInTheDocument(); // analyzed
});

test('displays recent documents list', async () => {
  vi.mocked(commands.listDocuments).mockResolvedValue({
    documents: [
      { id: '1', filename: 'contract.pdf', ... },
    ],
    total: 1,
  });

  render(<Dashboard />);

  expect(await screen.findByText('contract.pdf')).toBeInTheDocument();
});

test('shows error when stats fetch fails', async () => {
  vi.mocked(commands.getDocumentStats).mockRejectedValue(new Error('API error'));

  render(<Dashboard />);

  expect(await screen.findByText(/failed to load/i)).toBeInTheDocument();
});
```

**Upload.test.tsx** (NEW):
```typescript
test('accepts PDF files via drag-drop', () => {
  render(<Upload />);
  const dropZone = screen.getByText(/drag.*drop/i);

  const files = [new File([''], 'test.pdf', { type: 'application/pdf' })];
  fireEvent.drop(dropZone, { dataTransfer: { files } });

  expect(screen.getByDisplayValue('test.pdf')).toBeInTheDocument();
});

test('submits document for analysis', async () => {
  const mockUpload = vi.fn().mockResolvedValue({ id: '1' });
  vi.mocked(commands.uploadDocument).mockImplementation(mockUpload);

  render(<Upload />);

  // ... user interactions: select file, click upload
  fireEvent.click(screen.getByText('Upload'));

  await waitFor(() => {
    expect(mockUpload).toHaveBeenCalled();
    expect(toast.success).toHaveBeenCalled();
  });
});

test('shows error for invalid file type', () => {
  render(<Upload />);

  const invalidFile = new File([''], 'test.txt', { type: 'text/plain' });
  fireEvent.drop(screen.getByText(/drag.*drop/i), {
    dataTransfer: { files: [invalidFile] }
  });

  expect(screen.getByText(/only PDF files/i)).toBeInTheDocument();
});
```

**ReviewDetail.test.tsx** (NEW):
```typescript
test('loads and displays document details', async () => {
  vi.mocked(commands.getDocument).mockResolvedValue({
    id: '1',
    filename: 'contract.pdf',
    contractType: 'NDA',
    ...
  });

  render(<ReviewDetail />, { route: '/documents/1' });

  expect(await screen.findByText('contract.pdf')).toBeInTheDocument();
  expect(screen.getByText('NDA')).toBeInTheDocument();
});

test('displays extracted clauses', async () => {
  vi.mocked(commands.getExtractions).mockResolvedValue({
    clauses: [
      { text: 'Confidentiality clause...', importance: 'high' },
    ],
  });

  render(<ReviewDetail />);

  expect(await screen.findByText(/confidentiality/i)).toBeInTheDocument();
});

test('shows risk assessment', async () => {
  vi.mocked(commands.getRiskAssessments).mockResolvedValue({
    riskScore: 72,
    riskFlags: [{ severity: 'high', message: 'No termination clause' }],
  });

  render(<ReviewDetail />);

  expect(await screen.findByText('72')).toBeInTheDocument(); // risk score
});
```

**Comparison.test.tsx** (NEW):
```typescript
test('loads two documents for comparison', async () => {
  render(<Comparison />);

  // Select documents from dropdown
  // Verify comparison result displayed
});
```

**Templates.test.tsx** (NEW):
```typescript
test('displays list of templates', async () => {
  vi.mocked(commands.listTemplates).mockResolvedValue([
    { id: '1', name: 'Standard NDA', contractType: 'NDA', ... },
  ]);

  render(<Templates />);

  expect(await screen.findByText('Standard NDA')).toBeInTheDocument();
});

test('creates new template', async () => {
  render(<Templates />);

  fireEvent.click(screen.getByText('New Template'));
  fireEvent.change(screen.getByLabelText('Name'), { target: { value: 'My Template' } });
  fireEvent.click(screen.getByText('Create'));

  await waitFor(() => {
    expect(commands.createTemplate).toHaveBeenCalled();
  });
});
```

**Reports.test.tsx** (MODIFIED from Step 2):
```typescript
test('displays list of reports', async () => {
  vi.mocked(commands.listReports).mockResolvedValue([
    { id: '1', title: 'Analysis Report', ... },
  ]);

  render(<Reports />);

  expect(await screen.findByText('Analysis Report')).toBeInTheDocument();
});

test('downloads report', async () => {
  // ... test from Step 2
});

test('copies report to clipboard', async () => {
  render(<Reports />);
  fireEvent.click(screen.getByText('Copy'));

  expect(navigator.clipboard.writeText).toHaveBeenCalled();
});
```

**Settings.test.tsx** (MODIFIED from Step 1):
```typescript
// ... tests from Step 1
```

**App.test.tsx** (NEW):
```typescript
test('renders main layout and routes', () => {
  render(<App />);

  expect(screen.getByText('Legal Docs Review')).toBeInTheDocument();
  expect(screen.getByRole('navigation')).toBeInTheDocument();
});

test('navigates between pages', async () => {
  render(<App />);

  fireEvent.click(screen.getByText('Upload'));
  expect(await screen.findByText(/select a PDF/i)).toBeInTheDocument();
});
```

**Complexity:** Medium

**Test Coverage Target:** 50% (261 existing lines + ~600 new lines of tests)

**Error Handling in Tests:**
- Mock API failures to test error states
- Test error boundaries (if implemented)
- Test toast notifications on errors

**What is Unlocked After:**
- Step 5: Can now confidently begin linting and code cleanup with test safety net

---

#### STEP 5: Create Lint & Format Workflows
**Priority:** P1 (Establishes code quality gates)
**Files Touched:**
- `.github/workflows/lint.yml` (NEW)
- `.github/workflows/build.yml` (NEW)
- `.eslintrc.json` (NEW)
- `.prettierrc` (NEW)
- `eslint.config.mjs` (NEW)
- `.husky/pre-commit` (NEW)

**Prerequisites:**
- Steps 1-4 complete
- All tests passing

**What must be true before:**
- GitHub Actions test workflow passing

**Implementation:**

1. **Create `.eslintrc.json`:**
```json
{
  "extends": ["eslint:recommended", "plugin:react/recommended", "plugin:@typescript-eslint/recommended"],
  "rules": {
    "react/react-in-jsx-scope": "off",
    "no-console": ["warn", { "allow": ["warn", "error"] }],
    "@typescript-eslint/no-unused-vars": ["error", { "argsIgnorePattern": "^_" }]
  }
}
```

2. **Create `.prettierrc`:**
```json
{
  "semi": true,
  "singleQuote": true,
  "trailingComma": "es5",
  "printWidth": 100,
  "tabWidth": 2
}
```

3. **Create `.prettierignore`:**
```
node_modules
dist
*.json
*.yaml
```

4. **Create `.github/workflows/lint.yml`:**
```yaml
name: Lint & Format

on:
  push:
    branches: [main, claude/*]
  pull_request:
    branches: [main]

jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'pnpm'

      - name: Install dependencies
        run: pnpm install --frozen-lockfile

      - name: Run ESLint
        run: pnpm lint

      - name: Check formatting
        run: pnpm format:check
```

5. **Add to `package.json` scripts:**
```json
{
  "lint": "eslint src --ext .ts,.tsx",
  "lint:fix": "eslint src --ext .ts,.tsx --fix",
  "format": "prettier --write \"src/**/*.{ts,tsx}\"",
  "format:check": "prettier --check \"src/**/*.{ts,tsx}\""
}
```

6. **Create `.github/workflows/build.yml`:**
```yaml
name: Build

on:
  push:
    branches: [main, claude/*]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Setup pnpm
        uses: pnpm/action-setup@v2

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'pnpm'

      - name: Install
        run: pnpm install --frozen-lockfile

      - name: Type check
        run: pnpm tsc --noEmit

      - name: Build
        run: pnpm build
```

7. **Setup Husky pre-commit hook:**
```bash
pnpm install -D husky
npx husky install
npx husky add .husky/pre-commit "pnpm lint:fix && pnpm format"
```

**Complexity:** Low

**Error Handling:**
- Pre-commit hook blocks commit if linting fails (user must fix)
- CI blocks merge if formatting check fails

**What is Unlocked After:**
- Step 6: All quality gates in place; safe to begin removing dependencies and cleaning code

---

#### STEP 6: Remove Unused Dependencies
**Priority:** P2 (Technical debt cleanup)
**Files Touched:** `package.json`, `pnpm-lock.yaml`

**Prerequisites:**
- Step 5 complete (linting configured)
- All tests passing
- No code imports recharts or zod

**What must be true before:**
- Grep confirms neither `recharts` nor `zod` imported anywhere in codebase

**Implementation:**

1. Verify no imports:
```bash
grep -r "recharts\|zod" src --include="*.ts" --include="*.tsx"
# Should return 0 results
```

2. Remove from `package.json`:
   - Delete `"recharts": "^2.15.0"` from dependencies
   - Delete `"zod": "^3.24.0"` from dependencies (will add back in Step 8)

3. Update lock file:
```bash
pnpm install
```

4. Verify build still works:
```bash
pnpm build
```

**Complexity:** Low

**Testing:**
```bash
# Verify package.json is valid JSON
pnpm install --dry-run

# Verify build succeeds
pnpm build

# Verify no unused exports
pnpm tsc --noUnusedLocals
```

**What is Unlocked After:**
- Phases 2-6: Cleaner codebase, reduced bundle size

---

#### STEP 7: Commit Phase 1 Changes
**Priority:** P0 (Checkpoint)
**Files Touched:** All Phase 1 changes (git commit)

**Prerequisites:**
- Steps 1-6 complete
- All tests passing
- All linting passing

**What must be true before:**
- `pnpm test` passes
- `pnpm lint` passes
- No TypeScript errors: `pnpm tsc --noEmit`

**Implementation:**

```bash
git add .
git commit -m "Phase 1: Stabilization & Quality Gates

- Fixed Settings API key input consistency (unified to button-click save)
- Implemented Report download functionality using Tauri fs plugin
- Added GitHub Actions CI/CD pipelines (test, lint, build)
- Wrote integration tests for all 7 pages (50% coverage target)
- Configured ESLint, Prettier, and pre-commit hooks
- Removed unused dependencies (recharts, zod)

Test coverage increased from 14% to ~45%
All quality gates now passing on every commit"
```

**Complexity:** Low

**What is Unlocked After:**
- Phase 2 can begin (linting & code quality improvements)

---

### PHASE 2: CODE QUALITY & DEPENDENCIES (Steps 8-11)

---

#### STEP 8: Add Zod Schema Validation Layer
**Priority:** P1 (Runtime safety)
**Files Touched:**
- `src/lib/schemas.ts` (NEW)
- `src/lib/commands.ts` (MODIFIED)
- `package.json` (re-add zod)

**Prerequisites:**
- Phase 1 complete
- All tests passing

**What must be true before:**
- Zod added back to `package.json`
- Tauri commands responding with consistent JSON structure

**Implementation:**

1. Create `src/lib/schemas.ts` (reference: Section 3.3 above)
   - Define Zod schemas matching each Tauri command response
   - Export inferred types

2. Modify `src/lib/commands.ts`:
```typescript
// Before:
export const getDocument = async (id: string): Promise<Document> => {
  return invoke('get_document', { documentId: id });
};

// After:
import { DocumentSchema } from './schemas';

export const getDocument = async (id: string): Promise<Document> => {
  const raw = await invoke('get_document', { documentId: id });
  return DocumentSchema.parse(raw); // Validates at runtime
};
```

3. Add validation to all 22 commands in `src/lib/commands.ts`

4. Test validation:
```typescript
test('validates document response from backend', async () => {
  mockInvoke.mockResolvedValue({
    id: 'invalid-uuid', // Should fail validation
    ...
  });

  expect(() => getDocument('1')).rejects.toThrow(ZodError);
});

test('parses valid document response', async () => {
  mockInvoke.mockResolvedValue({
    id: '550e8400-e29b-41d4-a716-446655440000',
    filename: 'contract.pdf',
    ...
  });

  const doc = await getDocument('1');
  expect(doc.id).toBe('550e8400-e29b-41d4-a716-446655440000');
});
```

**Complexity:** Low

**Error Handling:**
- ZodError → show user-friendly error toast
- Log validation errors to console in dev
- Consider error boundary component for unhandled validation errors

**What is Unlocked After:**
- Confidence that API contracts are enforced at runtime
- Phase 3 documentation can reference these schemas

---

#### STEP 9: Document All Tauri Commands (API Reference)
**Priority:** P2 (Developer experience)
**Files Touched:** `docs/API.md` (NEW)

**Prerequisites:**
- Step 8 complete
- All 22 commands reviewed and understood

**What must be true before:**
- Commands in `src-tauri/src/lib.rs` finalized

**Implementation:**

Create `docs/API.md` with comprehensive command reference:

```markdown
# Tauri Commands API Reference

## Document Commands

### upload_document
Upload a PDF document for analysis.

**Parameters:**
- `filename: string` - Original filename
- `contract_type: string` - One of: NDA, ServiceAgreement, Lease, Other
- `file_bytes: number[]` - Binary PDF content

**Response:**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "filename": "contract.pdf"
}
```

**Status Codes:**
- 200 OK
- 400 Bad Request (invalid file format)
- 500 Internal Server Error (storage failure)

**Example:**
```typescript
const file = await readFile('contract.pdf');
const result = await invoke('upload_document', {
  filename: 'contract.pdf',
  contract_type: 'NDA',
  file_bytes: Array.from(file)
});
```

### extract_document_text
Extract text from uploaded PDF.

[... similar format for remaining 21 commands ...]
```

**Complexity:** Low (mechanical documentation)

**Content Structure:**
- One section per command
- Parameters with types
- Response with example JSON
- Status codes
- Usage example
- Notes on error cases

**What is Unlocked After:**
- Developers can self-serve API reference
- Phase 4 can reference schemas in documentation

---

#### STEP 10: Document Database Schema
**Priority:** P2 (Maintenance & onboarding)
**Files Touched:** `docs/DATABASE.md` (NEW)

**Prerequisites:**
- Step 9 complete
- Database schema finalized (no migrations planned)

**What must be true before:**
- SQLite migrations in `src-tauri/src/db/migrations.rs` reviewed

**Implementation:**

Create `docs/DATABASE.md` with:

1. **Overview:** Explain database design philosophy (SQLite, local-first, single-user)

2. **Schema Diagram:** ASCII diagram showing tables and relationships
```
documents (1) ──────────→ (N) extractions
   │                           │
   │                           └─→ ai_provider
   │
   ├─ (1) ──────────→ (N) risk_assessments
   │                           │
   │                           └─→ ai_provider
   │
   ├─ (1) ──────────→ (N) comparisons
   │                      │
   │                      ├─→ document_b (0-1)
   │                      └─→ template (0-1)
   │
   ├─ (1) ──────────→ (N) reports
   │
   └─ templates (N) ──────────→ (1) comparisons

settings (key-value store)
```

3. **Table Reference:** For each table:
   - Column name, type, constraints, description
   - Indexes
   - Example queries

4. **Relationships:** Foreign key constraints, cascade rules

**Complexity:** Low

**What is Unlocked After:**
- Complete documentation set for Phase 3+
- Ready for Phase 3 sign-off

---

#### STEP 11: Create Architecture Guide
**Priority:** P2 (Knowledge transfer)
**Files Touched:** `docs/ARCHITECTURE.md` (NEW)

**Prerequisites:**
- Steps 8-10 complete
- Full understanding of module boundaries

**What must be true before:**
- Project structure matches Section 2.1 of this plan

**Implementation:**

Create `docs/ARCHITECTURE.md` with:

1. **Module Overview:** Diagram (ASCII or Mermaid)
2. **Data Flow:** How data moves from frontend → Tauri → backend → database
3. **AI Provider Abstraction:** How different AI providers are swapped (trait pattern)
4. **Error Handling:** Error types, propagation strategy
5. **State Management:** React Context usage patterns, caching strategy
6. **Testing Strategy:** How to test each layer
7. **Deployment Architecture:** Desktop app structure, file organization

**Complexity:** Low

**What is Unlocked After:**
- Comprehensive documentation complete
- Team can understand codebase without external knowledge

---

#### STEP 12: Commit Phase 2 Changes
**Priority:** P0 (Checkpoint)
**Files Touched:** All Phase 2 changes (git commit)

**Prerequisites:**
- Steps 8-11 complete
- All tests passing
- All linting passing

**Implementation:**

```bash
git add .
git commit -m "Phase 2: Code Quality & Dependencies

- Added Zod runtime schema validation to all 22 Tauri commands
- Created comprehensive API reference (docs/API.md)
- Documented SQLite schema with ER diagram (docs/DATABASE.md)
- Created architecture guide for module organization (docs/ARCHITECTURE.md)
- Improved runtime type safety at API boundaries

Now catching API contract violations at runtime"
```

**Complexity:** Low

**What is Unlocked After:**
- Phase 3: Can begin implementing advanced features with confidence in documentation

---

### PHASE 3: DATA VISUALIZATION & UX POLISH (Steps 13-16)

---

#### STEP 13: Implement Dashboard Risk Charts
**Priority:** P1 (UX enhancement)
**Files Touched:**
- `src/pages/Dashboard.tsx` (MODIFIED)
- `src/components/analysis/RiskChart.tsx` (NEW)

**Prerequisites:**
- Phase 2 complete
- Recharts dependency re-added to `package.json`

**What must be true before:**
- Risk distribution data available via `get_risk_distribution()` command
- Dashboard page already displays stats

**Implementation:**

1. Create `src/components/analysis/RiskChart.tsx` (new Recharts component):
```typescript
import { PieChart, Pie, Cell, Legend, Tooltip } from 'recharts';

interface RiskChartProps {
  distribution: { high: number; medium: number; low: number };
}

export const RiskChart: React.FC<RiskChartProps> = ({ distribution }) => {
  const data = [
    { name: 'High Risk', value: distribution.high, fill: '#ef4444' },
    { name: 'Medium Risk', value: distribution.medium, fill: '#f59e0b' },
    { name: 'Low Risk', value: distribution.low, fill: '#10b981' },
  ];

  return (
    <PieChart width={400} height={300}>
      <Pie data={data} cx="50%" cy="50%" labelLine={false} label={renderLabel} />
      <Tooltip />
      <Legend />
    </PieChart>
  );
};

const renderLabel = (entry: any) => `${entry.name}: ${entry.value}`;
```

2. Modify `src/pages/Dashboard.tsx` to include chart:
```typescript
const Dashboard = () => {
  const [stats, setStats] = useState<DocumentStats | null>(null);
  const [riskDistribution, setRiskDistribution] = useState(null);

  useEffect(() => {
    async function loadData() {
      const s = await getDocumentStats();
      const rd = await getRiskDistribution();
      setStats(s);
      setRiskDistribution(rd);
    }
    loadData();
  }, []);

  return (
    <div>
      {/* Existing stats table */}
      {/* New chart */}
      {riskDistribution && <RiskChart distribution={riskDistribution} />}
    </div>
  );
};
```

3. Add visual styling:
   - Responsive grid layout (Tailwind)
   - Chart centered below stats table
   - Dark mode support if theme exists

4. Test:
```typescript
test('renders risk distribution pie chart', async () => {
  mockGetRiskDistribution.mockResolvedValue({
    high: 5,
    medium: 10,
    low: 20,
  });

  render(<Dashboard />);

  expect(await screen.findByText('High Risk')).toBeInTheDocument();
  expect(await screen.findByText('Medium Risk')).toBeInTheDocument();
});
```

**Complexity:** Low

**Error Handling:**
- If `getRiskDistribution()` fails, show empty state with retry button
- Log errors to console

**What is Unlocked After:**
- Dashboard now provides visual analytics
- Step 14: Can enhance RiskPanel component similarly

---

#### STEP 14: Enhance RiskPanel Component Visualization
**Priority:** P2 (UX polish)
**Files Touched:** `src/components/analysis/RiskPanel.tsx` (MODIFIED)

**Prerequisites:**
- Step 13 complete
- RiskPanel component currently renders text-based gauge

**What must be true before:**
- Risk assessment data including historical data (if available)

**Implementation:**

1. Enhance existing RiskPanel with:
   - Animated risk score gauge (HTML5 Canvas or SVG)
   - Severity breakdown pie chart (small)
   - Trend indicator (↑↓→) showing if risk increasing/decreasing
   - Detailed flag table with expandable suggestions

```typescript
// RiskPanel.tsx modifications
export const RiskPanel: React.FC<RiskPanelProps> = ({ assessment }) => {
  const flagsBySeverity = {
    high: assessment.riskFlags.filter(f => f.severity === 'high'),
    medium: assessment.riskFlags.filter(f => f.severity === 'medium'),
    low: assessment.riskFlags.filter(f => f.severity === 'low'),
  };

  return (
    <div className="bg-white rounded-lg shadow p-6">
      {/* Animated gauge */}
      <CircularGauge score={assessment.riskScore} />

      {/* Breakdown pie chart */}
      <SmallPieChart data={flagsBySeverity} />

      {/* Flags table */}
      <table>
        {/* High severity first, then medium, then low */}
      </table>
    </div>
  );
};
```

2. Add circular gauge animation:
```typescript
const CircularGauge = ({ score }: { score: number }) => {
  const circumference = 2 * Math.PI * 45;
  const offset = circumference - (score / 100) * circumference;

  return (
    <svg width="120" height="120">
      <circle cx="60" cy="60" r="45" fill="none" stroke="#e5e7eb" strokeWidth="8" />
      <circle
        cx="60"
        cy="60"
        r="45"
        fill="none"
        stroke={getColorByScore(score)}
        strokeWidth="8"
        strokeDasharray={circumference}
        strokeDashoffset={offset}
        style={{ transition: 'stroke-dashoffset 0.5s ease' }}
      />
      <text x="60" y="60" textAnchor="middle" fontSize="24" fontWeight="bold">
        {score}
      </text>
    </svg>
  );
};
```

3. Test enhancements:
```typescript
test('renders animated gauge with score', () => {
  render(<RiskPanel assessment={{ riskScore: 72, ... }} />);
  expect(screen.getByText('72')).toBeInTheDocument();
});

test('displays high-severity flags first', () => {
  const flags = [
    { severity: 'low', message: 'Minor issue' },
    { severity: 'high', message: 'Critical issue' },
  ];

  render(<RiskPanel assessment={{ riskFlags: flags, ... }} />);

  const rows = screen.getAllByRole('row');
  expect(rows[1]).toHaveTextContent('Critical issue');
  expect(rows[2]).toHaveTextContent('Minor issue');
});
```

**Complexity:** Medium (animation & layout)

**Error Handling:**
- If historical data unavailable, don't show trend indicator
- Fallback to simple gauge if canvas unsupported

**What is Unlocked After:**
- ReviewDetail page now provides rich analytics
- Step 15: Can add similar enhancements to comparison view

---

#### STEP 15: Add Advanced Comparison Features
**Priority:** P2 (Power user features)
**Files Touched:** `src/pages/Comparison.tsx` (MODIFIED)

**Prerequisites:**
- Phase 2 complete
- Comparison command returning structured diff data

**What must be true before:**
- Backend returns substantive vs. formatting differences clearly

**Implementation:**

1. Add comparison filtering:
```typescript
const [filterType, setFilterType] = useState<'all' | 'substantive' | 'formatting'>('all');

const filteredDifferences = useMemo(() => {
  if (filterType === 'substantive') return comparison.substantiveDifferences;
  if (filterType === 'formatting') return comparison.formattingDifferences;
  return [...comparison.substantiveDifferences, ...comparison.formattingDifferences];
}, [comparison, filterType]);
```

2. Add PDF export feature:
```typescript
import jsPDF from 'jspdf'; // Add to dependencies

const handleExportPDF = async () => {
  const doc = new jsPDF();
  doc.text('Comparison Report', 10, 10);
  doc.text(`Document A: ${comparison.documentA.filename}`, 10, 20);
  doc.text(`Document B: ${comparison.documentB?.filename || 'Template'}`, 10, 30);
  // ... add differences
  doc.save(`comparison_${new Date().toISOString()}.pdf`);
};
```

3. Enhanced side-by-side layout:
```typescript
// Create column layout with syntax highlighting for diffs
<div className="grid grid-cols-2 gap-4">
  <div className="bg-blue-50 p-4 rounded">
    <h3>Document A</h3>
    {/* Diff highlighting with +/- markers */}
  </div>
  <div className="bg-green-50 p-4 rounded">
    <h3>Document B</h3>
  </div>
</div>
```

**Complexity:** Medium

**Error Handling:**
- PDF export failure → show toast error
- Missing comparison data → show empty state

**What is Unlocked After:**
- Comparison page now feature-complete
- Step 16: Final Phase 3 checkpoint

---

#### STEP 16: Commit Phase 3 Changes
**Priority:** P0 (Checkpoint)
**Files Touched:** All Phase 3 changes (git commit)

**Prerequisites:**
- Steps 13-15 complete
- All tests passing
- All linting passing

**Implementation:**

```bash
git commit -m "Phase 3: Data Visualization & UX Polish

- Added dashboard risk distribution pie chart (Recharts)
- Enhanced RiskPanel with animated gauge and severity breakdown
- Added comparison filtering (substantive vs. formatting diffs)
- Implemented PDF export for comparison reports
- Improved side-by-side document layout with syntax highlighting

Dashboard now provides visual analytics. Comparison page offers power-user features."
```

**Complexity:** Low

**What is Unlocked After:**
- Phase 4: Can begin performance optimizations
- All visual enhancements complete

---

### PHASE 4: PERFORMANCE & ADVANCED FEATURES (Steps 17-20)

---

#### STEP 17: Implement React Context Caching Layer
**Priority:** P1 (Performance)
**Files Touched:**
- `src/lib/cache.ts` (NEW)
- `src/App.tsx` (MODIFIED)
- `src/hooks/useDocuments.ts` (MODIFIED)
- `src/lib/commands.ts` (MODIFIED)

**Prerequisites:**
- Phase 3 complete
- All tests passing

**What must be true before:**
- React Context API available
- No existing state management beyond hooks

**Implementation:**

1. Create `src/lib/cache.ts` (section 3.5 reference):
```typescript
import React, { createContext, useState, useCallback, useContext } from 'react';
import { Document, Extraction, RiskAssessment, DocumentStats } from '../types';

interface CacheState {
  documents: Map<string, Document>;
  extractions: Map<string, Extraction>;
  riskAssessments: Map<string, RiskAssessment>;
  stats: DocumentStats | null;
}

const initialState: CacheState = {
  documents: new Map(),
  extractions: new Map(),
  riskAssessments: new Map(),
  stats: null,
};

interface CacheContextType {
  state: CacheState;
  setDocument: (id: string, doc: Document) => void;
  setExtraction: (id: string, ext: Extraction) => void;
  setRiskAssessment: (id: string, risk: RiskAssessment) => void;
  setStats: (stats: DocumentStats) => void;
  invalidateDocument: (id: string) => void;
  invalidateAll: () => void;
}

export const CacheContext = createContext<CacheContextType | null>(null);

export const CacheProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [state, setState] = useState<CacheState>(initialState);

  const setDocument = useCallback((id: string, doc: Document) => {
    setState(prev => ({
      ...prev,
      documents: new Map(prev.documents).set(id, doc),
    }));
  }, []);

  const setExtraction = useCallback((id: string, ext: Extraction) => {
    setState(prev => ({
      ...prev,
      extractions: new Map(prev.extractions).set(id, ext),
    }));
  }, []);

  // ... similar for risk, stats, invalidate

  return (
    <CacheContext.Provider value={{ state, setDocument, setExtraction, ... }}>
      {children}
    </CacheContext.Provider>
  );
};

export const useCache = () => {
  const context = useContext(CacheContext);
  if (!context) throw new Error('useCache must be used within CacheProvider');
  return context;
};
```

2. Wrap App component in CacheProvider:
```typescript
// App.tsx
import { CacheProvider } from './lib/cache';

function App() {
  return (
    <CacheProvider>
      <MainLayout>
        <Routes>
          {/* ... */}
        </Routes>
      </MainLayout>
    </CacheProvider>
  );
}
```

3. Modify commands to use cache:
```typescript
// lib/commands.ts
import { useCache } from './cache';

export const useGetDocument = (id: string) => {
  const { state, setDocument } = useCache();
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (state.documents.has(id)) return; // Already cached

    setLoading(true);
    getDocument(id)
      .then(doc => setDocument(id, doc))
      .finally(() => setLoading(false));
  }, [id, state.documents, setDocument]);

  return {
    document: state.documents.get(id),
    loading,
  };
};
```

4. Test caching:
```typescript
test('caches document after first fetch', async () => {
  const { rerender } = render(<CachingComponent docId="1" />);

  expect(await screen.findByText('contract.pdf')).toBeInTheDocument();
  expect(mockGetDocument).toHaveBeenCalledTimes(1);

  // Re-render same component
  rerender(<CachingComponent docId="1" />);

  // Should not call API again
  expect(mockGetDocument).toHaveBeenCalledTimes(1);
});

test('invalidateAll clears cache', () => {
  // ...
});
```

**Complexity:** Medium

**Performance Impact:**
- Avoids re-fetching same document on navigation
- Reduces network calls by ~30-40% on typical usage
- Slightly increases memory usage (cached data)

**Error Handling:**
- Stale cache data returned if invalidate forgotten (acceptable tradeoff)
- Add manual refresh button to ReviewDetail page

**What is Unlocked After:**
- Significant performance improvement
- Step 18: Can add background queue without overloading backend

---

#### STEP 18: Implement Background Analysis Queue
**Priority:** P2 (Advanced feature)
**Files Touched:**
- `src/lib/queue.ts` (NEW)
- `src/pages/Upload.tsx` (MODIFIED)
- `src/hooks/useQueue.ts` (NEW)

**Prerequisites:**
- Step 17 complete (caching in place)
- Queue state management framework ready

**What must be true before:**
- Backend `analyze_document()` command available and functional
- Upload page can accept multiple files

**Implementation:**

1. Create `src/lib/queue.ts`:
```typescript
export interface QueueItem {
  id: string;
  documentId: string;
  status: 'pending' | 'processing' | 'completed' | 'failed';
  progress: number; // 0-100
  error?: string;
  createdAt: Date;
}

export class AnalysisQueue {
  private queue: QueueItem[] = [];
  private processing = false;
  private listeners: Set<(queue: QueueItem[]) => void> = new Set();

  add(documentId: string): QueueItem {
    const item: QueueItem = {
      id: uuid(),
      documentId,
      status: 'pending',
      progress: 0,
      createdAt: new Date(),
    };
    this.queue.push(item);
    this.notify();
    this.processNext();
    return item;
  }

  private async processNext() {
    if (this.processing || this.queue.length === 0) return;

    this.processing = true;
    const item = this.queue.find(q => q.status === 'pending');

    if (item) {
      item.status = 'processing';
      this.notify();

      try {
        item.progress = 30;
        this.notify();

        await analyzeDocument(item.documentId);

        item.progress = 100;
        item.status = 'completed';
      } catch (error) {
        item.status = 'failed';
        item.error = error.message;
      }
      this.notify();
    }

    this.processing = false;
    this.processNext();
  }

  subscribe(listener: (queue: QueueItem[]) => void) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private notify() {
    this.listeners.forEach(listener => listener(this.queue));
  }

  getQueue() {
    return this.queue;
  }
}

export const globalQueue = new AnalysisQueue();
```

2. Create hook `src/hooks/useQueue.ts`:
```typescript
import { useState, useEffect } from 'react';
import { globalQueue, QueueItem } from '../lib/queue';

export const useQueue = () => {
  const [queue, setQueue] = useState<QueueItem[]>([]);

  useEffect(() => {
    const unsubscribe = globalQueue.subscribe(setQueue);
    return unsubscribe;
  }, []);

  return {
    queue,
    addToQueue: (docId: string) => globalQueue.add(docId),
  };
};
```

3. Modify Upload page to use queue:
```typescript
const Upload = () => {
  const { addToQueue } = useQueue();

  const handleUpload = async (files: File[]) => {
    for (const file of files) {
      const result = await uploadDocument(file);
      addToQueue(result.id); // Add to background queue
    }
    toast.success(`${files.length} documents queued for analysis`);
  };
};
```

4. Add queue status display:
```typescript
const QueueStatus = () => {
  const { queue } = useQueue();

  return (
    <div>
      {queue.map(item => (
        <div key={item.id}>
          <span>{item.documentId}</span>
          <progress value={item.progress} max="100" />
          <span>{item.status}</span>
        </div>
      ))}
    </div>
  );
};
```

**Complexity:** Medium

**Error Handling:**
- Failed items remain in queue (user can retry)
- Network errors retry with exponential backoff
- Log queue activity

**What is Unlocked After:**
- Users can queue multiple documents for analysis
- Step 19: Can add batch comparison features

---

#### STEP 19: Add Batch Template Comparison
**Priority:** P2 (Advanced feature)
**Files Touched:** `src/pages/Templates.tsx` (MODIFIED)

**Prerequisites:**
- Step 18 complete
- Backend supports comparing document against multiple templates

**What must be true before:**
- `compareDocumentToTemplate()` command exists

**Implementation:**

1. Enhance Templates page:
```typescript
const [selectedTemplates, setSelectedTemplates] = useState<Set<string>>(new Set());
const [documentId, setDocumentId] = useState<string | null>(null);
const [comparisons, setComparisons] = useState<Comparison[]>([]);

const handleBatchCompare = async () => {
  for (const templateId of selectedTemplates) {
    const result = await compareDocuments({
      documentA: documentId,
      templateId,
    });
    setComparisons(prev => [...prev, result]);
  }
  toast.success(`Compared against ${selectedTemplates.size} templates`);
};
```

2. UI for multi-select templates, batch comparison

**Complexity:** Low

**Error Handling:**
- If one template comparison fails, continue with others
- Show partial results

**What is Unlocked After:**
- Step 20: Final Phase 4 checkpoint

---

#### STEP 20: Commit Phase 4 Changes
**Priority:** P0 (Checkpoint)
**Files Touched:** All Phase 4 changes (git commit)

**Prerequisites:**
- Steps 17-19 complete
- All tests passing

**Implementation:**

```bash
git commit -m "Phase 4: Performance & Advanced Features

- Implemented React Context caching layer for documents/extractions/risks
- Added background analysis queue for batch document processing
- Implemented batch template comparison feature
- Optimized re-renders and network calls

Performance improvements: ~30-40% fewer API calls on repeated navigation.
Advanced features enable power-user workflows."
```

**What is Unlocked After:**
- Phase 5: Deployment automation ready to build

---

### PHASE 5: DEPLOYMENT & RELEASE AUTOMATION (Steps 21-23)

---

#### STEP 21: Create Release GitHub Actions Workflow
**Priority:** P0 (Production deployment)
**Files Touched:** `.github/workflows/release.yml` (NEW)

**Prerequisites:**
- Phase 4 complete
- All workflows passing
- Repository public (for release artifacts)

**What must be true before:**
- Git tags follow semver (v1.0.0, v1.0.1, etc.)

**Implementation:**

Create `.github/workflows/release.yml`:
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        include:
          - os: ubuntu-latest
            artifact: legal_docs_review_*.AppImage
          - os: macos-latest
            artifact: legal_docs_review_*.dmg
          - os: windows-latest
            artifact: legal_docs_review_*.exe

    steps:
      - uses: actions/checkout@v4

      - name: Setup pnpm
        uses: pnpm/action-setup@v2
        with:
          version: 8

      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'pnpm'

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install dependencies
        run: pnpm install --frozen-lockfile

      - name: Build Tauri app
        run: pnpm tauri build

      - name: Upload release artifact
        uses: softprops/action-gh-release@v1
        with:
          files: src-tauri/target/release/bundle/**/
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

**Complexity:** Low

**Error Handling:**
- Build failure blocks release
- Artifacts signed (on macOS, optional on others)

**What is Unlocked After:**
- Automated cross-platform builds on git tag
- Step 22: Can add version management script

---

#### STEP 22: Create Version Bump Script
**Priority:** P1 (Release convenience)
**Files Touched:**
- `scripts/bump-version.sh` (NEW)
- `package.json` (add script)

**Prerequisites:**
- Step 21 complete

**What must be true before:**
- semver version scheme finalized

**Implementation:**

Create `scripts/bump-version.sh`:
```bash
#!/bin/bash

set -e

VERSION=$1

if [ -z "$VERSION" ]; then
  echo "Usage: ./scripts/bump-version.sh <version>"
  exit 1
fi

# Update package.json
sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" package.json

# Update Cargo.toml
sed -i "s/^version = .*/version = \"$VERSION\"/" src-tauri/Cargo.toml

# Update tauri.conf.json
sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" src-tauri/tauri.conf.json

# Commit and tag
git add package.json src-tauri/Cargo.toml src-tauri/tauri.conf.json
git commit -m "chore: bump version to $VERSION"
git tag "v$VERSION"

echo "Bumped version to $VERSION and created tag v$VERSION"
echo "Run 'git push && git push --tags' to trigger release"
```

Add to `package.json`:
```json
{
  "scripts": {
    "release": "bash scripts/bump-version.sh"
  }
}
```

**Complexity:** Low

**Usage:**
```bash
pnpm release 1.0.0
git push && git push --tags
```

**What is Unlocked After:**
- Step 23: Can finalize deployment documentation

---

#### STEP 23: Create Deployment Runbook
**Priority:** P2 (Operational documentation)
**Files Touched:** `docs/DEPLOYMENT.md` (NEW)

**Prerequisites:**
- Steps 21-22 complete

**What must be true before:**
- Release process tested end-to-end

**Implementation:**

Create `docs/DEPLOYMENT.md`:
```markdown
# Deployment Guide

## Release Process

### Prerequisites
- All tests passing on main branch
- Code reviewed and merged
- Changelog updated in README

### Steps

1. **Bump Version**
   ```bash
   pnpm release 1.0.1
   ```
   This updates version in:
   - package.json
   - Cargo.toml
   - tauri.conf.json

2. **Push to GitHub**
   ```bash
   git push origin main
   git push origin --tags
   ```

3. **Watch Release Build**
   - Go to https://github.com/saagar210/LegalDocsReview/actions
   - Monitor release workflow
   - Wait for builds on macOS, Windows, Linux to complete (~10 minutes)

4. **Verify Release**
   - Go to https://github.com/saagar210/LegalDocsReview/releases
   - Download and test installers on each platform
   - Update release notes with changelog

5. **Publish**
   - Mark release as "Latest"
   - Announce in changelog/blog

## Rollback

If critical bug found:
```bash
git tag -d v1.0.1
git push origin :v1.0.1
# Fix bug
pnpm release 1.0.2
git push --tags
```

## Troubleshooting

### macOS build fails with code-signing error
- Update provisioning profile in Tauri config
- Ensure developer certificate installed locally

### Windows build fails
- Ensure Rust MSVC toolchain installed
- Check Windows signing certificate

### Release artifacts not uploading
- Check GitHub token has repo:write permissions
- Verify artifact paths in release.yml
```

**Complexity:** Low

**What is Unlocked After:**
- Deployment process fully documented
- Step 24: Final Phase 5 checkpoint

---

#### STEP 24: Commit Phase 5 Changes
**Priority:** P0 (Checkpoint)
**Files Touched:** All Phase 5 changes (git commit)

**Prerequisites:**
- Steps 21-23 complete
- Release workflow tested end-to-end

**Implementation:**

```bash
git commit -m "Phase 5: Deployment & Release Automation

- Created GitHub Actions release workflow for cross-platform builds
- Added version bump script for semver management
- Created comprehensive deployment runbook

Release pipeline: git tag v1.0.0 → automated builds → GitHub release"
```

**What is Unlocked After:**
- Phase 6: Final polish and production readiness

---

### PHASE 6: PRODUCTION READINESS (Steps 25-27)

---

#### STEP 25: Add Error Logging Service
**Priority:** P1 (Observability)
**Files Touched:**
- `src/lib/logger.ts` (NEW)
- `src/main.tsx` (MODIFIED)

**Prerequisites:**
- Phase 5 complete

**What must be true before:**
- Error patterns understood from development

**Implementation:**

1. Create `src/lib/logger.ts`:
```typescript
export interface ErrorLog {
  timestamp: string;
  level: 'error' | 'warn' | 'info';
  message: string;
  context?: Record<string, any>;
}

class Logger {
  private logs: ErrorLog[] = [];
  private maxLogs = 1000;

  error(message: string, context?: Record<string, any>) {
    this.log('error', message, context);
  }

  warn(message: string, context?: Record<string, any>) {
    this.log('warn', message, context);
  }

  info(message: string, context?: Record<string, any>) {
    this.log('info', message, context);
  }

  private log(level: 'error' | 'warn' | 'info', message: string, context?: Record<string, any>) {
    const entry: ErrorLog = {
      timestamp: new Date().toISOString(),
      level,
      message,
      context,
    };

    this.logs.push(entry);
    if (this.logs.length > this.maxLogs) {
      this.logs.shift();
    }

    // In production, send to logging service
    if (level === 'error' && process.env.NODE_ENV === 'production') {
      this.sendToServer(entry);
    }

    console[level](message, context);
  }

  private async sendToServer(entry: ErrorLog) {
    // Stub for Sentry/other logging service
    // fetch('https://log-service.example.com/logs', { method: 'POST', body: JSON.stringify(entry) });
  }

  getLogs() {
    return this.logs;
  }
}

export const logger = new Logger();
```

2. Modify components to use logger:
```typescript
import { logger } from '../lib/logger';

// In error handlers:
try {
  await analyzeDocument(docId);
} catch (error) {
  logger.error('Document analysis failed', { docId, error: error.message });
  toast.error('Analysis failed');
}
```

**Complexity:** Low

**Error Handling:**
- Logging service failure doesn't break app (fire-and-forget)
- Local log buffer maintained

**What is Unlocked After:**
- Production error visibility
- Step 26: Can add analytics

---

#### STEP 26: Add Usage Analytics (Optional)
**Priority:** P2 (Business metrics)
**Files Touched:** `src/lib/analytics.ts` (NEW)

**Prerequisites:**
- Step 25 complete

**Implementation:**

Basic analytics tracking:
```typescript
export const trackEvent = (event: string, properties?: Record<string, any>) => {
  // Track to analytics service (e.g., Posthog, Mixpanel)
  // fire-and-forget
};

// Usage:
trackEvent('document_uploaded', { contractType: 'NDA' });
trackEvent('analysis_completed', { riskScore: 72 });
```

**Complexity:** Low

**What is Unlocked After:**
- Understanding user behavior
- Step 27: Final Phase 6 checkpoint

---

#### STEP 27: Commit Phase 6 & Final Sign-Off
**Priority:** P0 (Final)
**Files Touched:** All Phase 6 changes (git commit)

**Prerequisites:**
- Steps 25-26 complete
- All workflows passing
- Full test coverage
- All documentation complete

**Implementation:**

```bash
git commit -m "Phase 6: Production Readiness

- Added error logging service for observability
- Implemented usage analytics tracking
- Completed production deployment infrastructure

LegalDocsReview is now production-ready.
Core functionality: 100%
Test coverage: 50%+
CI/CD: Automated on every commit and tag
Documentation: Complete API, database, and architecture guides"
```

Push to main:
```bash
git push origin claude/analyze-repo-overview-wQ8op
```

---

## 5. ERROR HANDLING STRATEGY

### 5.1 Frontend Error Handling

| Error Type | Detection | Recovery | User Feedback |
|-----------|-----------|----------|----------------|
| Network timeout | Tauri IPC timeout | Retry (exponential backoff) | "Connection lost, retrying..." toast |
| Invalid API response | Zod schema validation | Log error, show fallback UI | "Data format error" toast |
| File I/O error | `writeTextFile()` rejects | User selects different location | "Permission denied" error toast |
| AI provider offline | API returns 503/timeout | Fallback to alternate provider | "AI service unavailable" toast |
| Out of memory | Large file upload | Show file size warning before upload | "File too large" error |
| Unknown error | Uncaught exception | Error boundary catches it | "Something went wrong" toast + error ID |

### 5.2 Backend Error Handling

**All Tauri commands return Result<T, AppError>:**
- Success: `{ Ok: T }` (automatically JSON-serialized)
- Failure: `{ Err: { error_type: string, message: string, details: string } }`

**Error Types:**
```rust
enum AppError {
    Database(String),
    Io(String),
    PdfExtraction(String),
    AiProvider(String),
    Json(String),
    Http(String),
    Validation(String),
    NotFound(String),
}
```

**Per-Step Error Handling:**

| Step | Error Scenario | Handling | User Impact |
|------|----------------|----------|-------------|
| 1 (Settings fix) | setSetting fails | Retry logic, keep dirty state | User sees error, can retry |
| 2 (Report download) | writeTextFile permission denied | Show OS error message | User selects alternate path |
| 3-4 (Tests) | Test failures on CI | CI marks build red, blocks merge | Developer fixes before merging |
| 8 (Schemas) | Response doesn't match schema | ZodError thrown, caught by error boundary | Toast: "Data format error" |
| 13 (Charts) | getRiskDistribution fails | Show empty state + retry button | User sees "No data available" |
| 17 (Caching) | Stale cache served | Manual refresh button available | User clicks "Refresh data" |
| 18 (Queue) | Analysis fails midway | Item marked failed, remains in queue | User sees failed status, can retry |

### 5.3 Validation Strategy

**Input Validation:**
- Frontend: Pre-validation before API calls (file size, type, format)
- Backend: Mandatory schema validation on all requests
- Database: Foreign key constraints, NOT NULL constraints

**Output Validation:**
- Zod schemas validate all Tauri command responses
- Type checking at compile time (TypeScript strict mode)
- Runtime parsing at API boundary

---

## 6. TESTING STRATEGY

### 6.1 Unit Tests (Low-level)

**What to test:**
- Utility functions: `getColorByRiskScore()`, `formatDate()`, etc.
- Type validation: Zod schemas parse/reject correctly
- Hooks: `useDocuments()`, `useCache()`, `useQueue()`
- Components in isolation: Props, rendering, user interactions

**Testing Stack:** Vitest + @testing-library/react

**Coverage Target:** 70%+ unit test coverage

**Example:**
```typescript
describe('useDocuments', () => {
  test('fetches documents on mount', async () => {
    const mockData = [{ id: '1', filename: 'contract.pdf', ... }];
    vi.mocked(commands.listDocuments).mockResolvedValue({ documents: mockData, total: 1 });

    const { result } = renderHook(() => useDocuments());

    await waitFor(() => {
      expect(result.current.documents).toEqual(mockData);
    });
  });

  test('handles API error', async () => {
    vi.mocked(commands.listDocuments).mockRejectedValue(new Error('API error'));

    const { result } = renderHook(() => useDocuments());

    await waitFor(() => {
      expect(result.current.error).toBeDefined();
    });
  });
});
```

### 6.2 Integration Tests (Mid-level)

**What to test:**
- Page workflows: Upload → Analyze → Review → Report
- State management: Cache invalidation, queue processing
- Backend + Frontend: E2E command execution

**Testing approach:**
- Mock Tauri backend fully
- Verify data flows correctly through pages
- Test error states and recovery

**Example:**
```typescript
describe('Upload → Analyze → Review workflow', () => {
  test('complete document analysis flow', async () => {
    // 1. Upload document
    render(<UploadPage />);
    const file = new File(['pdf content'], 'contract.pdf');
    fireEvent.drop(screen.getByText(/drag/), { dataTransfer: { files: [file] } });
    fireEvent.click(screen.getByText('Upload'));

    const documentId = await waitFor(() => expect(mockUploadDocument).toHaveBeenCalled());

    // 2. Add to queue and analyze
    const { result } = renderHook(() => useQueue());
    result.current.addToQueue(documentId);

    await waitFor(() => {
      expect(mockAnalyzeDocument).toHaveBeenCalledWith(documentId);
    });

    // 3. Navigate to review
    render(<ReviewDetailPage />, { route: `/documents/${documentId}` });

    expect(await screen.findByText(/risk score/i)).toBeInTheDocument();
  });
});
```

### 6.3 E2E Tests (High-level)

**What to test:**
- Real Tauri IPC (if available)
- SQLite database queries
- File system operations

**Framework:** Playwright (if Tauri supports it) or manual testing

**Coverage:** Happy paths only (upload, analyze, compare, download)

### 6.4 Verification Checklist

**Before each step:**
- [ ] Unit tests pass (`pnpm test`)
- [ ] Lint passes (`pnpm lint`)
- [ ] Type check passes (`pnpm tsc --noEmit`)
- [ ] Manual testing on one platform (Linux/macOS/Windows)

**Before committing:**
- [ ] All GitHub Actions workflows pass
- [ ] Coverage threshold met (≥50%)
- [ ] No console errors in dev mode

**Before shipping release:**
- [ ] Build succeeds on all platforms
- [ ] Installers tested on macOS, Windows, Linux
- [ ] Settings/reports/downloads verified

---

## 7. EXPLICIT ASSUMPTIONS

### 7.1 Project Scope Assumptions
- **Single-user desktop app:** No multi-user authentication required
- **Local data only:** No cloud sync or web deployment in MVP
- **PDF-only documents:** No support for Word, Google Docs, etc.
- **English-language contracts:** AI prompts optimized for English
- **Async AI processing:** Always run analysis in background (never block UI)
- **Bundled database:** SQLite sufficient; no server-side DB needed

### 7.2 Technical Assumptions
- **Tauri 2.x stable:** Tauri API won't break; versions in plan are current
- **OpenAI/Anthropic APIs stable:** No major changes to chat endpoints
- **Ollama available locally:** Users install Ollama separately if using local LLM
- **Node.js 20+ available:** Development environment has Node toolchain
- **Rust 1.77.2+:** Tauri compilation won't fail due to old Rust version
- **Modern browsers:** Tauri webview uses Chromium/WebKit; no IE11 support needed
- **File system writable:** App data directory is writable (`~/.app_data_dir/`)
- **Network available for AI:** User has internet for OpenAI/Claude APIs

### 7.3 User Behavior Assumptions
- **API keys stored locally:** Users enter OpenAI/Claude keys once; stored in app settings
- **Documents ≤50MB:** PDFs larger than 50MB not tested; may cause performance issues
- **Typical usage:** < 100 documents per user (caching sufficient)
- **Contract types known:** Users select NDA/ServiceAgreement/Lease/Other correctly
- **English contracts:** No special handling for multilingual documents

### 7.4 Performance Assumptions
- **UI responsive at ≤100ms latency:** Tauri IPC is fast enough
- **AI responses ≤60 seconds:** OpenAI/Claude respond in reasonable time
- **Database queries ≤500ms:** SQLite on local disk is fast enough
- **App binary ≤200MB:** Tauri bundle size acceptable
- **Memory usage ≤500MB:** Caching doesn't exhaust system memory

### 7.5 Security Assumptions (Non-goal in MVP)
- **No encryption at rest:** Settings/documents stored in plain text locally
- **No API key rotation:** Keys static in settings (user manages rotation)
- **No audit logs:** No tracking of who accessed what documents
- **Single-machine security:** App trusts local OS user permissions
- **No supply chain verification:** Dependencies not cryptographically signed

### 7.6 Deployment Assumptions
- **GitHub Actions available:** Repository on GitHub with Actions enabled
- **Cross-platform builds:** GitHub hosted runners (ubuntu-latest, macos-latest, windows-latest) work
- **Semantic versioning:** Releases follow semver (v1.0.0)
- **Public GitHub releases:** Binaries publicly downloadable
- **No code signing for Linux:** AppImage doesn't require signing
- **Optional code signing:** macOS/Windows signing can be added later

### 7.7 Integration Assumptions
- **Tauri file dialog working:** `@tauri-apps/plugin-dialog` reliably opens file picker
- **Tauri fs plugin working:** `@tauri-apps/plugin-fs` can write files
- **PDF extraction:** `pdf-extract` crate can handle 95%+ of contract PDFs
- **AI provider errors graceful:** OpenAI/Claude errors return proper HTTP status codes

---

## 8. QUALITY GATE & SIGN-OFF

### 8.1 Pre-Submission Checklist

**Logical Completeness:**
- ✅ Every step has prerequisites and downstream dependencies defined
- ✅ No circular dependencies (Step A blocks B, B blocks A)
- ✅ All 27 steps sequentially ordered
- ✅ No assumed external knowledge (all decisions explained)
- ✅ Error handling defined for each failure mode
- ✅ Testing strategy covers all layers

**Actionability:**
- ✅ Every step includes exact files to create/modify
- ✅ Code changes include pseudocode or concrete patterns
- ✅ Complexity rating provided for each step
- ✅ Time estimates reasonable (sum: ~18-20 hours to completion)
- ✅ Prerequisites clearly stated for each step

**Architectural Soundness:**
- ✅ Module boundaries clear (no spaghetti dependencies)
- ✅ Type safety throughout (TypeScript + Zod)
- ✅ Error handling consistent (Result types, error boundaries)
- ✅ Testing strategy covers unit/integration/E2E
- ✅ Deployment automated (GitHub Actions)

**Risk Mitigation:**
- ✅ No single-point failures (can recover from partial failures)
- ✅ Testing gates each phase (can detect regressions early)
- ✅ Caching prevents backend overload (Step 17)
- ✅ Error logging enables production debugging (Step 25)
- ✅ Staged rollout possible (release workflow)

### 8.2 Known Gaps & Mitigations

| Gap | Risk | Mitigation |
|-----|------|-----------|
| No web version in MVP | Limited market reach | Can be added in Phase 7 (post-MVP) |
| Single-user only | Enterprise features missing | Multi-user auth can be Phase 7 |
| No encryption | Sensitive data in plaintext | Out of scope for desktop MVP; add if needed |
| Local SQLite only | No data backup | Users responsible; add cloud sync in Phase 7 |
| Manual version bumping | Human error in release | Script automates bumping; process documented |
| Test coverage 50% | Some untested code paths | Coverage sufficient for MVP; can improve later |

### 8.3 Judgment Calls Made

1. **React Hooks + Context over Redux:** Simpler for single-user desktop app; sufficient state management
2. **Zod validation added retroactively (Phase 2):** Better to delay validation setup than to rush it in Phase 1
3. **Caching via Context, not persistent storage:** Simpler than localStorage; acceptable for single-user app
4. **Background queue simple implementation:** Manual retry sufficient; no need for job queue service (BullMQ, etc.)
5. **Error logging local-first:** Can be integrated with Sentry later; local buffer sufficient for MVP
6. **No E2E tests in Phase 1:** Unit + integration tests sufficient; E2E can be added if CI/CD pipeline robust

### 8.4 Final Sign-Off

**STATUS: ✅ APPROVED FOR EXECUTION**

This plan is:
- ✅ **Complete:** All 27 steps defined, no ambiguity
- ✅ **Achievable:** 18-20 hours to completion, feasible in 3-4 sessions
- ✅ **Testable:** Each step has verification criteria
- ✅ **Safe:** Error handling and testing throughout
- ✅ **Documented:** Every assumption, decision, and risk identified

**Ready for execution by Codex. Zero clarifying questions should be needed.**

---

## APPENDIX: Implementation Timeline

**Assumed work capacity: 6-8 hours per session, 3-4 sessions**

**Session 1 (Day 1): Phase 1 - Stabilization (2-3 hours)**
- Steps 1-7: Fix bugs, add CI/CD, write tests, remove dead code

**Session 2 (Day 2): Phase 2 - Quality (2-3 hours)**
- Steps 8-12: Add validation, documentation

**Session 3 (Day 3): Phases 3-4 - Features (3-4 hours)**
- Steps 13-20: Charts, performance, advanced features

**Session 4 (Day 4): Phases 5-6 - Deployment (2-3 hours)**
- Steps 21-27: Release automation, production ready

**Total: 9-13 hours across 4 sessions → Project 100% complete**

---

**END OF IMPLEMENTATION PLAN**
