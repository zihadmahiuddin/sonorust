import {
  ReactFlow,
  ReactFlowProvider,
  ConnectionLineType,
  MarkerType,
  useNodesState,
  useEdgesState,
  useReactFlow,
  useNodesInitialized,
  type Node,
  type Edge,
  Handle,
  Position,
  type NodeProps,
} from "@xyflow/react";
import "@xyflow/react/dist/style.css";
import dagre from "@dagrejs/dagre";
import type {
  BasicBlock,
  CompilationResult,
  Instruction,
} from "sonorust-debugger-wasm";
import { memo, useEffect, useMemo, useRef, type Ref } from "react";
import InstructionLine from "./InstructionLine";
import { useVMStore } from "../stores/vmStore";

export const BasicBlockNode = memo(({ data }: NodeProps<BasicBlockNode>) => {
  const { block, comments, currentLineRef, instructions, label } = data;

  return (
    <div className="bg-[#1e2530] border border-[#3b4252] rounded-md shadow-lg min-w-65 text-left font-mono text-sm overflow-hidden">
      <Handle
        type="target"
        position={Position.Top}
        className="bg-[#4fb3d9]! w-2! h-2!"
      />
      {label && (
        <div className="bg-[#2e3440] px-3 py-1 text-[#88c0d0] border-b border-[#3b4252] text-xs font-bold font-sans">
          {label}
        </div>
      )}

      <div className="p-2 space-y-0.5">
        {instructions.map((_, idx) => {
          const addr = block.startPc + idx;
          return (
            <InstructionLine
              key={addr}
              addr={addr}
              comment={comments.get(addr)}
              currentLineRef={currentLineRef}
              // in graph view the label is already shown on top of the node
              // so we don't need to show it again as part of the InstructionLine
              label={null}
            />
          );
        })}
      </div>

      <Handle
        type="source"
        position={Position.Bottom}
        className="bg-[#4fb3d9]! w-2! h-2!"
      />
    </div>
  );
});

BasicBlockNode.displayName = "BasicBlockNode";

const nodeTypes = {
  basicBlock: BasicBlockNode,
};

function GraphDisassembly({
  comments,
  currentLineRef,
}: {
  comments: Map<number, string>;
  currentLineRef: Ref<HTMLDivElement>;
}) {
  return (
    <ReactFlowProvider>
      <GraphDisassemblyInner
        comments={comments}
        currentLineRef={currentLineRef}
      />
    </ReactFlowProvider>
  );
}

function GraphDisassemblyInner({
  comments,
  currentLineRef,
}: {
  comments: Map<number, string>;
  currentLineRef: Ref<HTMLDivElement>;
}) {
  const compilationResult = useVMStore((s) => s.compilationResult);

  const raw = useMemo(() => {
    const nodes: Node[] = compilationResult.basicBlocks.map((block) => ({
      id: block.id.toString(),
      type: "basicBlock",
      position: { x: 0, y: 0 },
      style: { width: NODE_WIDTH },
      data: {
        block,
        label: compilationResult.labels.get(block.startPc),
        instructions: compilationResult.instructions.slice(
          block.startPc,
          block.endPc,
        ),
        comments,
        currentLineRef,
      },
    }));

    const edges: Edge[] = [];
    compilationResult.basicBlocks.forEach((block) => {
      if (!block.edge) return;
      if (typeof block.edge !== "object") return;

      if ("unconditional" in block.edge) {
        edges.push({
          id: `e-${block.id}-${block.edge.unconditional.targetBlockId}`,
          source: block.id.toString(),
          target: block.edge.unconditional.targetBlockId.toString(),
          label: "jump",
          style: { stroke: "#4fb3d9", strokeWidth: 2 },
          markerEnd: { type: MarkerType.ArrowClosed, color: "#4fb3d9" },
        });
      } else if ("conditional" in block.edge) {
        const { trueBlockId, falseBlockId } = block.edge.conditional;
        edges.push({
          id: `e-${block.id}-${trueBlockId}-t`,
          source: block.id.toString(),
          target: trueBlockId.toString(),
          label: "true",
          style: { stroke: "#10b981", strokeWidth: 2 },
          markerEnd: { type: MarkerType.ArrowClosed, color: "#10b981" },
        });
        edges.push({
          id: `e-${block.id}-${falseBlockId}-f`,
          source: block.id.toString(),
          target: falseBlockId.toString(),
          label: "false",
          style: { stroke: "#ef4444", strokeWidth: 2 },
          markerEnd: { type: MarkerType.ArrowClosed, color: "#ef4444" },
        });
      } else if ("fallthrough" in block.edge) {
        edges.push({
          id: `e-${block.id}-${block.edge.fallthrough.targetBlockId}`,
          source: block.id.toString(),
          target: block.edge.fallthrough.targetBlockId.toString(),
          style: { stroke: "#4fb3d9", strokeWidth: 2 },
          markerEnd: { type: MarkerType.ArrowClosed, color: "#4fb3d9" },
        });
      }
    });
    return { nodes, edges };
  }, [compilationResult]);

  const [nodes, setNodes, onNodesChange] = useNodesState<Node>(raw.nodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState<Edge>(raw.edges);

  const { getNodes, fitView } = useReactFlow();
  const nodesInitialized = useNodesInitialized();
  const layoutedFor = useRef<CompilationResult | null>(null);

  useEffect(() => {
    layoutedFor.current = null;
    setNodes(raw.nodes);
    setEdges(raw.edges);
  }, [raw, setNodes, setEdges]);

  useEffect(() => {
    if (!nodesInitialized) return;
    if (layoutedFor.current === compilationResult) return;

    const layouted = getLayoutedElements(getNodes(), raw.edges);
    layoutedFor.current = compilationResult;
    setNodes(layouted.nodes);
    setEdges(layouted.edges);
    requestAnimationFrame(() => fitView({ duration: 200 }));
  }, [
    nodesInitialized,
    compilationResult,
    raw.edges,
    getNodes,
    setNodes,
    setEdges,
    fitView,
  ]);

  return (
    <ReactFlow
      nodes={nodes}
      edges={edges}
      onNodesChange={onNodesChange}
      onEdgesChange={onEdgesChange}
      connectionLineType={ConnectionLineType.SmoothStep}
      defaultEdgeOptions={{ type: "smoothstep" }}
      nodeTypes={nodeTypes}
      fitView
      colorMode="system"
    />
  );
}

type BasicBlockNode = Node<
  {
    label?: string;
    block: BasicBlock;
    comments: Map<number, string>;
    currentLineRef: Ref<HTMLDivElement>;
    instructions: Instruction[];
  },
  "basicBlock"
>;

const NODE_WIDTH = 280;

function getLayoutedElements(nodes: Node[], edges: Edge[]) {
  const dagreGraph = new dagre.graphlib.Graph().setDefaultEdgeLabel(() => ({}));
  dagreGraph.setGraph({ rankdir: "TB", nodesep: 60, ranksep: 80 });

  nodes.forEach((node) => {
    const width = node.measured?.width ?? NODE_WIDTH;
    const height = node.measured?.height ?? 80;
    dagreGraph.setNode(node.id, { width, height });
  });

  edges.forEach((edge) => {
    dagreGraph.setEdge(edge.source, edge.target);
  });

  dagre.layout(dagreGraph);

  const layoutedNodes = nodes.map((node) => {
    const { x, y } = dagreGraph.node(node.id);
    const width = node.measured?.width ?? NODE_WIDTH;
    const height = node.measured?.height ?? 80;
    return {
      ...node,
      position: { x: x - width / 2, y: y - height / 2 },
    };
  });

  return { nodes: layoutedNodes, edges };
}

export default memo(GraphDisassembly);
