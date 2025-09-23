import { useCallback, useEffect, useMemo, useRef, useState } from "react";
import type { ReactNode } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  TS_AST_NODES,
  type TsAstNode,
  type TsAstStatus,
  type TsAstPriority,
  type IrReference,
  type IrSpec,
  type IrSpecField,
} from "./data/tsAstNodes";

// --- Design tokens (colors given by user) ---
const BG = "#2B2429"; // app background
const ACCENT = "#7A3980"; // micro elements / focus

// --- Small helpers ---
const cls = (...s: Array<string | false | null | undefined>) => s.filter(Boolean).join(" ");

type IrDocField = {
  name: string;
  signature: string;
};

type IrDocVariant = {
  name: string;
  signature: string;
  fields: IrDocField[];
};

type IrDocNode = {
  name: string;
  kind: string;
  signature: string;
  fields: IrDocField[];
  variants: IrDocVariant[];
};

type IrDocsPayload = {
  generated_at: number;
  docs: IrDocNode[];
  json: string;
};

// --- App ---
export default function App() {
  const [active, setActive] = useState<"stats" | "ide" | "docs">("ide");

  return (
    <div className="w-screen h-screen flex" style={{ backgroundColor: BG, color: "#EAE6EA" }}>
      <SideNav active={active} onChange={setActive} />
      <div className="flex-1 flex flex-col">
        <TopBar />
        {active === "stats" ? (
          <StatsPlaceholder />
        ) : active === "ide" ? (
          <IDEWorkspace />
        ) : (
          <DocsWorkspace />
        )}
      </div>
    </div>
  );
}

// --- Navigation ---
function SideNav({ active, onChange }: { active: "stats" | "ide" | "docs"; onChange: (t: "stats" | "ide" | "docs") => void }) {
  return (
    <aside className="h-full w-64 border-r border-[#3a3338] flex flex-col" style={{ backgroundColor: "#241F23" }}>
      <div className="px-5 py-6 text-sm tracking-widest uppercase text-[#B8AEB6]">ts2rust</div>
      <nav className="px-2 space-y-2">
        <NavItem label="Stats" active={active === "stats"} onClick={() => onChange("stats")} />
        <NavItem label="IDE" active={active === "ide"} onClick={() => onChange("ide")} />
        <NavItem label="Docs" active={active === "docs"} onClick={() => onChange("docs")} />
      </nav>
      <div className="mt-auto p-4 text-xs text-[#8f848d]">v0.1 · Tauri</div>
    </aside>
  );
}

function NavItem({ label, active, onClick }: { label: string; active?: boolean; onClick?: () => void }) {
  return (
    <button
      onClick={onClick}
      className={cls(
        "w-full text-left px-4 py-3 rounded-xl transition-colors",
        active ? "bg-[#2d272c] text-white" : "hover:bg-[#2d272c] text-[#CFC6CE]"
      )}
      style={{ outlineColor: ACCENT }}
    >
      <span className={cls("text-sm", active && "font-medium")}>{label}</span>
    </button>
  );
}

// --- Top bar ---
function TopBar() {
  return (
    <header className="h-14 border-b border-[#3a3338] flex items-center justify-between px-4">
      <div className="flex items-center gap-3">
        <span className="text-[#CFC6CE]">Dashboard</span>
        <span className="text-xs px-2 py-1 rounded-full" style={{ backgroundColor: "#342d32", color: ACCENT }}>alpha</span>
      </div>
      <div className="flex items-center gap-2">
        <input
          placeholder="Search"
          className="bg-[#231d22] border border-[#3a3338] rounded-lg px-3 py-1.5 text-sm outline-none focus:ring-2"
          style={{ caretColor: ACCENT, boxShadow: `0 0 0 0 rgba(0,0,0,0)` }}
        />
      </div>
    </header>
  );
}

// --- Stats (placeholder page) ---
function StatsPlaceholder() {
  return (
    <div className="flex-1 grid md:grid-cols-2 xl:grid-cols-3 gap-6 p-6">
      {["Total builds", "Errors", "Warnings", "Avg time", "Cache hits", "IR nodes"].map((t) => (
        <Card key={t}>
          <div className="text-sm text-[#B8AEB6]">{t}</div>
          <div className="text-3xl mt-2 text-[#EDE7EE]">—</div>
        </Card>
      ))}
      <Card className="xl:col-span-2">
        <div className="text-sm text-[#B8AEB6]">Timeline</div>
        <div className="h-48 mt-4 rounded-lg border border-[#3a3338] grid place-items-center text-[#8f848d]">
          chart placeholder
        </div>
      </Card>
    </div>
  );
}

// --- Docs Workspace ---
function DocsWorkspace() {
  const [selectedId, setSelectedId] = useState<string>("program");
  const [expanded, setExpanded] = useState<Record<string, boolean>>(() => defaultExpandedState(TS_AST_NODES));
  const [payload, setPayload] = useState<IrDocsPayload | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const syncDocs = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await invoke<IrDocsPayload>("sync_ir_docs");
      setPayload(result);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    syncDocs();
  }, [syncDocs]);

  const docLookup = useMemo(() => {
    const map = new Map<string, IrDocNode>();
    payload?.docs.forEach((doc) => {
      map.set(doc.name, doc);
    });
    return map;
  }, [payload]);

  const statusMap = useMemo(() => computeStatusMap(TS_AST_NODES, docLookup), [docLookup]);
  const selectedNode = useMemo(() => findAstNode(TS_AST_NODES, selectedId) ?? null, [selectedId]);
  const selectedStatus = selectedNode ? statusMap[selectedNode.id] : undefined;

  const lastSync = useMemo(() => {
    if (!payload) return null;
    return new Date(payload.generated_at * 1000).toLocaleTimeString();
  }, [payload]);

  const handleToggle = useCallback((id: string) => {
    setExpanded((prev) => ({
      ...prev,
      [id]: !(prev[id] ?? false),
    }));
  }, []);

  const handleSelect = useCallback((id: string) => {
    setSelectedId(id);
  }, []);

  const irRefs = selectedNode?.ir ?? [];

  const referenceSpecs = irRefs.map((ref) => {
    const doc = docLookup.get(ref.type);
    const variant = ref.variant ? doc?.variants.find((v) => v.name === ref.variant) : undefined;
    const implemented = ref.variant ? Boolean(variant) : Boolean(doc);
    const actual = buildActualSpec(doc, variant);
    const ideal = buildIdealSpec(ref, actual);
    const fallback = fallbackSpec(ref);
    return { ref, implemented, actual, ideal, fallback };
  });

  return (
    <div className="flex-1 flex min-h-0">
      <section className="w-80 border-r border-[#3a3338] flex flex-col" style={{ backgroundColor: "#241F23" }}>
        <div className="px-4 py-3 border-b border-[#3a3338] text-sm uppercase tracking-wide text-[#B8AEB6]">Docs</div>
        <div className="flex-1 overflow-auto p-2">
          <DocsTree
            nodes={TS_AST_NODES}
            statuses={statusMap}
            expanded={expanded}
            selected={selectedId}
            onToggle={handleToggle}
            onSelect={handleSelect}
          />
        </div>
      </section>
      <section className="flex-1 flex flex-col">
        <Toolbar>
          <span className="text-sm text-[#B8AEB6]">IR Docs</span>
          <div className="ml-auto flex items-center gap-3">
            {lastSync ? <span className="text-xs text-[#8f848d]">Last synced: {lastSync}</span> : null}
            {error ? <span className="text-xs text-[#F199A1]">Error: {error}</span> : null}
            <ActionButton onClick={syncDocs} disabled={loading}>
              {loading ? "Syncing…" : "Sync"}
            </ActionButton>
          </div>
        </Toolbar>
        <div className="flex-1 overflow-auto p-4 space-y-4">
          {selectedNode ? (
            <>
              <Card className="space-y-3">
                <div className="flex items-center gap-3">
                  <div className="text-lg text-[#EDE7EE]">{selectedNode.title}</div>
                  <span
                    className="text-[10px] px-2 py-0.5 rounded-full uppercase tracking-wide"
                    style={{ backgroundColor: "#342d32", color: statusColor(selectedStatus ?? statusForPriority(selectedNode.priority)) }}
                  >
                    {statusLabel(selectedStatus ?? statusForPriority(selectedNode.priority))}
                  </span>
                  <span className="text-xs text-[#8f848d]">{selectedNode.swcKind}</span>
                </div>
                {selectedNode.description ? (
                  <div className="text-sm text-[#CFC6CE]">{selectedNode.description}</div>
                ) : null}
                {irRefs.length ? (
                  <div>
                    <div className="text-xs text-[#8f848d] uppercase tracking-wide">IR references</div>
                    <ul className="mt-1 space-y-1 text-sm text-[#EDE7EE]">
                      {irRefs.map((ref) => (
                        <li key={`${ref.type}-${ref.variant ?? "base"}`}>
                          {ref.type}
                          {ref.variant ? `::${ref.variant}` : ""}
                        </li>
                      ))}
                    </ul>
                  </div>
                ) : (
                  <div className="text-sm text-[#8f848d]">No IR linkage defined yet.</div>
                )}
              </Card>
              {referenceSpecs.length ? (
                referenceSpecs.map((info) => (
                  <div key={`${info.ref.type}-${info.ref.variant ?? "base"}`} className="grid gap-3 md:grid-cols-2">
                    <SpecCard
                      title="Current IR"
                      spec={info.implemented ? info.actual : null}
                      placeholder="—"
                    />
                    <SpecCard
                      title="Target IR"
                      spec={info.ideal ?? info.actual ?? info.fallback}
                      placeholder="Not specified"
                    />
                  </div>
                ))
              ) : (
                <SpecCard
                  title="Target IR"
                  spec={null}
                  placeholder="Configure IR references on the left to track coverage."
                />
              )}
            </>
          ) : (
            <Card>
              <div className="text-sm text-[#CFC6CE]">Select an AST node to inspect the IR mapping.</div>
            </Card>
          )}
        </div>
      </section>
    </div>
  );
}

function DocsTree({
  nodes,
  statuses,
  expanded,
  selected,
  onToggle,
  onSelect,
  depth = 0,
}: {
  nodes: TsAstNode[];
  statuses: Record<string, TsAstStatus>;
  expanded: Record<string, boolean>;
  selected: string;
  onToggle: (id: string) => void;
  onSelect: (id: string) => void;
  depth?: number;
}) {
  return (
    <div className="space-y-1">
      {nodes.map((node) => {
        const hasChildren = !!node.children?.length;
        const open = expanded[node.id] ?? depth === 0;
        const isSelected = selected === node.id;
        const nodeStatus = statuses[node.id] ?? statusForPriority(node.priority);

        return (
          <div key={node.id}>
            <div className="flex items-center">
              {hasChildren ? (
                <button
                  onClick={() => onToggle(node.id)}
                  className="w-6 h-6 flex items-center justify-center text-xs text-[#8f848d] hover:text-[#EDE7EE]"
                >
                  {open ? "−" : "+"}
                </button>
              ) : (
                <span className="w-6" />
              )}
              <button
                onClick={() => onSelect(node.id)}
                className={cls(
                  "flex-1 flex items-center gap-2 px-2 py-1 rounded-lg transition-colors text-left",
                  isSelected ? "bg-[#2d272c]" : "hover:bg-[#2d272c]"
                )}
              >
                <span className="w-2 h-2 rounded-full" style={{ backgroundColor: statusColor(nodeStatus) }} />
                <span className="text-sm text-[#EDE7EE]">{node.title}</span>
                <span className="ml-auto text-[10px] uppercase tracking-wide text-[#8f848d]">{node.swcKind}</span>
              </button>
            </div>
            {hasChildren && open ? (
              <div className="ml-6 mt-1">
                <DocsTree
                  nodes={node.children!}
                  statuses={statuses}
                  expanded={expanded}
                  selected={selected}
                  onToggle={onToggle}
                  onSelect={onSelect}
                  depth={depth + 1}
                />
              </div>
            ) : null}
          </div>
        );
      })}
    </div>
  );
}

function buildActualSpec(doc?: IrDocNode, variant?: IrDocVariant): IrSpec | null {
  if (!doc) return null;
  if (variant) {
    return {
      title: `${doc.name}::${variant.name}`,
      signature: variant.signature,
      fields: variant.fields.map(convertDocField),
    };
  }
  return {
    title: doc.name,
    signature: doc.signature,
    fields: doc.fields.map(convertDocField),
  };
}

function buildIdealSpec(ref: IrReference, actual: IrSpec | null): IrSpec | null {
  if (ref.ideal) {
    return {
      title: ref.ideal.title ?? (ref.variant ? `${ref.type}::${ref.variant}` : ref.type),
      summary: ref.ideal.summary,
      signature: ref.ideal.signature ?? actual?.signature,
      fields: ref.ideal.fields?.map((field) => ({ ...field })),
    };
  }
  return actual;
}

function fallbackSpec(ref: IrReference): IrSpec {
  return {
    title: ref.variant ? `${ref.type}::${ref.variant}` : ref.type,
  };
}

function convertDocField(field: IrDocField): IrSpecField {
  return {
    name: field.name,
    signature: field.signature,
  };
}

function SpecCard({ title, spec, placeholder }: { title: string; spec: IrSpec | null; placeholder?: string }) {
  return (
    <Card className="space-y-3">
      <div className="text-sm text-[#B8AEB6]">{title}</div>
      {spec ? (
        <div className="space-y-2">
          {spec.title ? <div className="text-sm text-[#EDE7EE] font-medium">{spec.title}</div> : null}
          {spec.summary ? <div className="text-xs text-[#CFC6CE]">{spec.summary}</div> : null}
          {spec.signature ? (
            <div className="text-xs font-mono text-[#9b8fa0] bg-[#1F1A1E] border border-[#3a3338] rounded-md px-2 py-1">
              {spec.signature}
            </div>
          ) : null}
          {spec.fields?.length ? (
            <div>
              <div className="text-xs text-[#8f848d] uppercase tracking-wide">Fields</div>
              <ul className="mt-1 space-y-1 text-sm text-[#CFC6CE]">
                {spec.fields.map((field) => (
                  <li key={field.name}>
                    <span className="text-[#EDE7EE]">{field.name}</span>
                    <span className="text-[#8f848d]">: {field.signature}</span>
                    {field.note ? <div className="text-[11px] text-[#8f848d]">{field.note}</div> : null}
                  </li>
                ))}
              </ul>
            </div>
          ) : null}
        </div>
      ) : (
        <div className="text-sm text-[#8f848d]">{placeholder ?? "—"}</div>
      )}
    </Card>
  );
}

function statusColor(status: TsAstStatus) {
  if (status === "implemented") return "#3BA55B";
  if (status === "missing-mvp") return "#F55A68";
  return "#E3B341";
}

function statusLabel(status: TsAstStatus) {
  if (status === "implemented") return "Covered in IR";
  if (status === "missing-mvp") return "MVP gap";
  return "Backlog gap";
}

function statusForPriority(priority: TsAstPriority): TsAstStatus {
  return priority === "mvp" ? "missing-mvp" : "missing-later";
}

function findAstNode(nodes: TsAstNode[], id: string): TsAstNode | undefined {
  for (const node of nodes) {
    if (node.id === id) return node;
    if (node.children) {
      const found = findAstNode(node.children, id);
      if (found) return found;
    }
  }
  return undefined;
}

function defaultExpandedState(nodes: TsAstNode[], depth = 0, acc: Record<string, boolean> = {}): Record<string, boolean> {
  for (const node of nodes) {
    if (node.children?.length) {
      acc[node.id] = depth < 2;
      defaultExpandedState(node.children, depth + 1, acc);
    }
  }
  return acc;
}

const STATUS_WEIGHT: Record<TsAstStatus, number> = {
  implemented: 0,
  "missing-later": 1,
  "missing-mvp": 2,
};

function mergeStatuses(statuses: TsAstStatus[]): TsAstStatus {
  return statuses.reduce((worst, current) =>
    STATUS_WEIGHT[current] > STATUS_WEIGHT[worst] ? current : worst
  );
}

function hasReference(ref: IrReference, docs: Map<string, IrDocNode>): boolean {
  const entry = docs.get(ref.type);
  if (!entry) return false;
  if (!ref.variant) return true;
  return entry.variants.some((variant) => variant.name === ref.variant);
}

function computeStatusMap(nodes: TsAstNode[], docs: Map<string, IrDocNode>): Record<string, TsAstStatus> {
  const map: Record<string, TsAstStatus> = {};

  const visit = (node: TsAstNode): TsAstStatus => {
    const references = node.ir ?? [];
    let ownStatus: TsAstStatus | null = null;

    if (references.length) {
      const implemented = references.every((ref) => hasReference(ref, docs));
      ownStatus = implemented ? "implemented" : statusForPriority(node.priority);
    } else if (!node.children?.length) {
      ownStatus = statusForPriority(node.priority);
    }

    const childStatuses = node.children?.map(visit) ?? [];
    const toMerge = [
      ...(ownStatus ? [ownStatus] : []),
      ...childStatuses,
    ];

    const finalStatus = toMerge.length
      ? mergeStatuses(toMerge)
      : statusForPriority(node.priority);

    map[node.id] = finalStatus;
    return finalStatus;
  };

  nodes.forEach(visit);
  return map;
}

// --- IDE Workspace ---
function IDEWorkspace() {
  const [leftMode, setLeftMode] = useState<"code" | "ast">("code");
  const [rightMode, setRightMode] = useState<"ir" | "rust" | "rust-ast">("ir");

  const [code, setCode] = useState<string>(`// TypeScript demo\nfunction add(a: number, b: number) {\n  return a + b;\n}\n\nconsole.log(add(2, 3));`);

  // hover sync between left AST and right panel
  const [hoveredId, setHoveredId] = useState<string | null>(null);

  // compile/run placeholders until real Tauri wiring lands
  const [isTsCompiling, setIsTsCompiling] = useState(false);
  const [tsResponse, setTsResponse] = useState<string>("Waiting for execution");
  const [tsError, setTsError] = useState<string | null>(null);

  const [isRustRunning, setIsRustRunning] = useState(false);
  const [rustResponse, setRustResponse] = useState<string>("Waiting for execution");
  const [rustError, setRustError] = useState<string | null>(null);

  // Placeholders to wire later with Tauri
  async function compileIR() {
    // const result = await invoke<string>("ts_to_ir", { code });
    // setIr(result)
  }

  async function makeAST() {
    // const result = await invoke<string>("ts_to_ast", { code });
    // setAstJson(result)
  }

  async function handleCompileTs() {
    setIsTsCompiling(true);
    setTsError(null);
    try {
      const result = await invoke<string>("compile_ts", { source: code });
      setTsResponse(result || "Placeholder: backend wiring pending.");
    } catch (error) {
      setTsError(error instanceof Error ? error.message : String(error));
      setTsResponse("Placeholder: AST will be available after integration.");
    } finally {
      setIsTsCompiling(false);
    }
  }

  async function handleRunRust() {
    setIsRustRunning(true);
    setRustError(null);
    try {
      const result = await invoke<string>("run_rust", { source: code });
      setRustResponse(result || "Placeholder: execution output will appear later.");
    } catch (error) {
      setRustError(error instanceof Error ? error.message : String(error));
      setRustResponse("Hello world");
    } finally {
      setIsRustRunning(false);
    }
  }

  useEffect(() => {
    // fire and forget stubs when code changes (debounced in real app)
  }, [code]);

  return (
    <div className="flex-1 flex min-h-0">
      {/* Left: Editor / AST */}
      <section className="w-1/2 min-w-[420px] border-r border-[#3a3338] flex flex-col">
        <Toolbar>
          <Segmented
            value={leftMode}
            onChange={(v) => setLeftMode(v as any)}
            options={[
              { label: "Code (TS)", value: "code" },
              { label: "AST (TS)", value: "ast" },
            ]}
          />
          <div className="ml-auto" />
        </Toolbar>

        {leftMode === "code" ? (
          <Editor code={code} onChange={setCode} />
        ) : (
          <div className="flex-1 flex flex-col p-4 gap-4">
            <div className="flex-1 min-h-0 overflow-auto">
              <ASTView
                source={code}
                hoveredId={hoveredId}
                onHover={setHoveredId}
                onCompile={handleCompileTs}
                compiling={isTsCompiling}
              />
            </div>
          <ResponseCard
            title="Response (TS)"
              content={tsResponse}
              loading={isTsCompiling}
              error={tsError}
              placeholder="—"
            />
          </div>
        )}
      </section>

      {/* Right: IR / Rust / Rust AST */}
      <section className="flex-1 flex flex-col">
        <Toolbar>
          <Segmented
            value={rightMode}
            onChange={(v) => setRightMode(v as any)}
            options={[
              { label: "IR", value: "ir" },
              { label: "Rust", value: "rust" },
              { label: "Rust AST", value: "rust-ast" },
            ]}
          />
        </Toolbar>

        <RightPane
          mode={rightMode}
          hoveredId={hoveredId}
          onHover={setHoveredId}
          onRunRust={handleRunRust}
          running={isRustRunning}
          rustOutput={rustResponse}
          rustError={rustError}
        />
      </section>
    </div>
  );
}

// --- Editor (simple, replace later with CodeMirror/Monaco) ---
function Editor({ code, onChange }: { code: string; onChange: (v: string) => void }) {
  const ref = useRef<HTMLTextAreaElement | null>(null);
  return (
    <div className="flex-1 p-4">
      <div className="rounded-2xl border border-[#3a3338] overflow-hidden shadow-inner" style={{ boxShadow: "inset 0 1px 0 rgba(255,255,255,0.03)" }}>
        <div className="px-3 py-2 text-xs flex items-center gap-2 border-b border-[#3a3338]" style={{ background: "#241F23" }}>
          <Dot /> <Dot /> <Dot />
          <span className="ml-2 text-[#9b8fa0]">main.ts</span>
        </div>
        <textarea
          ref={ref}
          spellCheck={false}
          value={code}
          onChange={(e) => onChange(e.target.value)}
          className="w-full h-[calc(100vh-220px)] bg-[#1F1A1E] text-[#EDE7EE] outline-none p-4 text-sm leading-6 font-mono"
          style={{ caretColor: ACCENT }}
        />
      </div>
    </div>
  );
}

function Dot() {
  return <span className="w-2 h-2 rounded-full" style={{ backgroundColor: ACCENT }} />;
}

// --- AST view (mock tree so you can already wire hover sync) ---
function ASTView({
  source,
  hoveredId,
  onHover,
  onCompile,
  compiling,
}: {
  source: string;
  hoveredId: string | null;
  onHover: (id: string | null) => void;
  onCompile: () => void;
  compiling: boolean;
}) {
  // A very naive fake AST from lines. Replace with real one from Tauri later.
  const nodes = useMemo(() => createMockAst(source), [source]);
  return (
    <Card className="p-0">
      <div className="px-4 py-3 border-b border-[#3a3338] flex items-center justify-between text-sm text-[#B8AEB6]">
        <span>AST (mock)</span>
        <ActionButton onClick={onCompile} disabled={compiling}>
          {compiling ? "Compiling…" : "Compile"}
        </ActionButton>
      </div>
        <ul className="p-3 space-y-1 text-sm">
          {nodes.map((n) => (
            <li key={n.id}>
              <AstRow node={n} level={0} hoveredId={hoveredId} onHover={onHover} />
            </li>
          ))}
        </ul>
    </Card>
  );
}

function AstRow({ node, level, hoveredId, onHover }: { node: AstNode; level: number; hoveredId: string | null; onHover: (id: string | null) => void }) {
  const isActive = hoveredId === node.id;
  return (
    <div>
      <div
        onMouseEnter={() => onHover(node.id)}
        onMouseLeave={() => onHover(null)}
        className={cls(
          "flex items-center gap-2 px-2 py-1 rounded-lg transition-colors cursor-default",
          isActive ? "bg-[#2d272c]" : "hover:bg-[#2d272c]"
        )}
        style={{ borderLeft: `2px solid ${isActive ? ACCENT : "transparent"}`, paddingLeft: 8 + level * 12 }}
      >
        <span className="text-xs uppercase tracking-wide text-[#9b8fa0]">{node.type}</span>
        <span className="text-[#EDE7EE]">{node.label}</span>
      </div>
      {node.children?.map((c) => (
        <AstRow key={c.id} node={c} level={level + 1} hoveredId={hoveredId} onHover={onHover} />
      ))}
    </div>
  );
}

// --- Right panel ---
function RightPane({
  mode,
  hoveredId,
  onHover,
  onRunRust,
  running,
  rustOutput,
  rustError,
}: {
  mode: "ir" | "rust" | "rust-ast";
  hoveredId: string | null;
  onHover: (id: string | null) => void;
  onRunRust: () => void;
  running: boolean;
  rustOutput: string;
  rustError: string | null;
}) {
  return (
    <div className="flex-1 p-4 overflow-auto">
      <div className="space-y-4">
        <Card className="p-0">
          <div className="px-4 py-3 border-b border-[#3a3338] flex items-center justify-between">
            <span className="text-sm text-[#B8AEB6]">{modeTitle(mode)}</span>
            <div className="flex items-center gap-2">
              <span className="text-[10px] px-2 py-0.5 rounded-full" style={{ backgroundColor: "#342d32", color: ACCENT }}>
                stub
              </span>
              {mode === "rust-ast" ? (
                <ActionButton onClick={onRunRust} disabled={running}>
                  {running ? "Running…" : "Run"}
                </ActionButton>
              ) : null}
            </div>
          </div>
          {mode === "ir" ? <IRStub hoveredId={hoveredId} onHover={onHover} /> : null}
          {mode === "rust" ? <RustStub /> : null}
          {mode === "rust-ast" ? <RustAstStub /> : null}
        </Card>
        {mode === "rust-ast" ? (
          <ResponseCard
            title="Response (Rust)"
            content={rustOutput}
            loading={running}
            error={rustError}
            placeholder="—"
          />
        ) : null}
      </div>
    </div>
  );
}

function modeTitle(m: string) {
  if (m === "ir") return "Intermediate Representation";
  if (m === "rust") return "Rust";
  return "Rust AST";
}

function IRStub({ hoveredId, onHover }: { hoveredId: string | null; onHover: (id: string | null) => void }) {
  // This mirrors the mock AST ids to demonstrate highlight sync
  const items = [
    { id: "node-0", text: "Function add(a, b)" },
    { id: "node-1", text: "Return +" },
    { id: "node-2", text: "Call console.log" },
  ];
  return (
    <div className="p-3 space-y-1 text-sm">
      {items.map((it) => (
        <div
          key={it.id}
          onMouseEnter={() => onHover(it.id)}
          onMouseLeave={() => onHover(null)}
          className={cls(
            "px-2 py-1 rounded-lg",
            hoveredId === it.id ? "bg-[#2d272c]" : "hover:bg-[#2d272c]"
          )}
          style={{ borderLeft: `2px solid ${hoveredId === it.id ? ACCENT : "transparent"}` }}
        >
          {it.text}
        </div>
      ))}
      <div className="mt-4 text-xs text-[#8f848d]">Real IR output from your compiler will appear here.</div>
    </div>
  );
}

function RustStub() {
  return (
    <pre className="p-3 text-sm text-[#EDE7EE] whitespace-pre-wrap">
{`fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\nfn main() {\n    println!("{}", add(2, 3));\n}`} 
    </pre>
  );
}

function RustAstStub() {
  return (
    <div className="p-3 text-sm text-[#B8AEB6]">Rust AST placeholder.</div>
  );
}

// --- UI primitives ---
function Card({ children, className = "", style }: { children: any; className?: string; style?: React.CSSProperties }) {
  return (
    <div
      className={cls(
        "rounded-2xl border border-[#3a3338] bg-[#241F23] p-4 shadow-[0_1px_0_rgba(255,255,255,0.03)_inset]",
        className
      )}
      style={style}
    >
      {children}
    </div>
  );
}

function ResponseCard({
  title,
  content,
  loading,
  error,
  placeholder = "—",
  className,
}: {
  title: string;
  content: string;
  loading?: boolean;
  error?: string | null;
  placeholder?: string;
  className?: string;
}) {
  return (
    <Card className={className}>
      <div className="text-sm text-[#B8AEB6]">{title}</div>
      <div className="mt-2 text-sm text-[#EDE7EE] whitespace-pre-wrap font-mono leading-6">
        {loading ? "Loading…" : content || placeholder}
      </div>
      {error ? <div className="mt-3 text-xs text-[#F199A1] break-words">Error: {error}</div> : null}
    </Card>
  );
}

function Toolbar({ children }: { children: any }) {
  return (
    <div className="h-12 px-4 border-b border-[#3a3338] flex items-center gap-3" style={{ backgroundColor: "#241F23" }}>
      {children}
    </div>
  );
}

function Segmented({ value, onChange, options }: { value: string; onChange: (v: string) => void; options: { label: string; value: string }[] }) {
  return (
    <div className="inline-flex items-center p-1 rounded-xl border border-[#3a3338] bg-[#1F1A1E]">
      {options.map((o) => {
        const active = value === o.value;
        return (
          <button
            key={o.value}
            onClick={() => onChange(o.value)}
            className={cls(
              "text-xs px-3 py-1.5 rounded-lg transition-colors",
              active ? "text-white" : "text-[#CFC6CE] hover:text-white"
            )}
            style={{ backgroundColor: active ? "#2d272c" : "transparent", outlineColor: ACCENT, boxShadow: active ? `0 0 0 1px ${ACCENT} inset` : undefined }}
          >
            {o.label}
          </button>
        );
      })}
    </div>
  );
}

function ActionButton({ children, onClick, disabled }: { children: ReactNode; onClick?: () => void; disabled?: boolean }) {
  return (
    <button
      type="button"
      onClick={disabled ? undefined : onClick}
      disabled={disabled}
      className={cls(
        "text-xs px-3 py-1.5 rounded-lg border border-[#3a3338] transition-colors",
        disabled ? "bg-[#2b252a] text-[#8f848d] cursor-not-allowed" : "bg-[#2d272c] text-[#EDE7EE] hover:bg-[#332c31]"
      )}
      style={{ outlineColor: ACCENT }}
    >
      {children}
    </button>
  );
}

// --- Mock AST builder used for hover demo ---

type AstNode = { id: string; type: string; label: string; children?: AstNode[] };

function createMockAst(src: string): AstNode[] {
  const lines = src.split(/\n+/).filter(Boolean);
  const root: AstNode[] = [];
  let id = 0;
  for (const line of lines) {
    if (/function\s+/.test(line)) {
      const name = (line.match(/function\s+(\w+)/) || [])[1] || "fn";
      root.push({ id: `node-${id++}`, type: "FunctionDecl", label: name, children: [
        { id: `node-${id++}`, type: "Params", label: line.includes("(") ? line.split("(")[1].split(")")[0] : "", children: [] },
        { id: `node-${id++}`, type: "Body", label: "{…}", children: [] },
      ]});
    } else if (/console\./.test(line)) {
      root.push({ id: `node-${id++}`, type: "ExprStmt", label: line.trim() });
    }
  }
  return root.length ? root : [{ id: "node-0", type: "Program", label: "(empty)" }];
}
