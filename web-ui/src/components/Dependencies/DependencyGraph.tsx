import React, { useEffect, useRef, useState } from 'react';
import * as d3 from 'd3';
import { Task, Priority } from '@/types';
import Card from '@/components/UI/Card';
import Button from '@/components/UI/Button';
import { ZoomIn, ZoomOut, RotateCcw, Maximize2 } from 'lucide-react';

interface DependencyGraphProps {
  tasks: Task[];
  onTaskClick?: (task: Task) => void;
  className?: string;
}

interface GraphNode extends d3.SimulationNodeDatum {
  id: string;
  task: Task;
  level: number;
}

interface GraphLink extends d3.SimulationLinkDatum<GraphNode> {
  source: string | GraphNode;
  target: string | GraphNode;
}

const DependencyGraph: React.FC<DependencyGraphProps> = ({
  tasks,
  onTaskClick,
  className = '',
}) => {
  const svgRef = useRef<SVGSVGElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 800, height: 600 });
  const [zoom, setZoom] = useState(1);
  
  // Color scheme for different priorities
  const priorityColors = {
    [Priority.Critical]: '#ef4444',
    [Priority.High]: '#f97316', 
    [Priority.Medium]: '#eab308',
    [Priority.Low]: '#22c55e',
  };

  // Status colors
  const statusColors = {
    todo: '#6b7280',
    'in-progress': '#3b82f6',
    completed: '#10b981',
    blocked: '#ef4444',
  };

  useEffect(() => {
    const updateDimensions = () => {
      if (containerRef.current) {
        const { width, height } = containerRef.current.getBoundingClientRect();
        setDimensions({ width: width || 800, height: height || 600 });
      }
    };

    updateDimensions();
    window.addEventListener('resize', updateDimensions);
    return () => window.removeEventListener('resize', updateDimensions);
  }, []);

  useEffect(() => {
    if (!svgRef.current || tasks.length === 0) return;

    // Clear previous content
    d3.select(svgRef.current).selectAll('*').remove();

    // Prepare data
    const nodes: GraphNode[] = tasks.map(task => ({
      id: task.id.toString(),
      task,
      level: calculateTaskLevel(task, tasks),
    }));

    const links: GraphLink[] = [];
    tasks.forEach(task => {
      task.dependencies.forEach(depId => {
        if (tasks.find(t => t.id === depId)) {
          links.push({
            source: depId.toString(),
            target: task.id.toString(),
          });
        }
      });
    });

    // Create SVG
    const svg = d3.select(svgRef.current);
    const g = svg.append('g');

    // Add zoom behavior
    const zoomBehavior = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on('zoom', (event) => {
        g.attr('transform', event.transform);
        setZoom(event.transform.k);
      });

    svg.call(zoomBehavior);

    // Create force simulation
    const simulation = d3.forceSimulation<GraphNode>(nodes)
      .force('link', d3.forceLink<GraphNode, GraphLink>(links)
        .id(d => d.id)
        .distance(100)
        .strength(0.8))
      .force('charge', d3.forceManyBody().strength(-300))
      .force('center', d3.forceCenter(dimensions.width / 2, dimensions.height / 2))
      .force('collision', d3.forceCollide().radius(40));

    // Create arrow markers for links
    svg.append('defs').selectAll('marker')
      .data(['dependency'])
      .enter().append('marker')
      .attr('id', d => d)
      .attr('viewBox', '0 -5 10 10')
      .attr('refX', 25)
      .attr('refY', 0)
      .attr('markerWidth', 6)
      .attr('markerHeight', 6)
      .attr('orient', 'auto')
      .append('path')
      .attr('d', 'M0,-5L10,0L0,5')
      .attr('fill', '#64748b');

    // Create links
    const link = g.append('g')
      .selectAll('line')
      .data(links)
      .enter().append('line')
      .attr('stroke', '#64748b')
      .attr('stroke-width', 2)
      .attr('stroke-opacity', 0.6)
      .attr('marker-end', 'url(#dependency)');

    // Create nodes
    const node = g.append('g')
      .selectAll('g')
      .data(nodes)
      .enter().append('g')
      .attr('class', 'node')
      .style('cursor', 'pointer')
      .call(d3.drag<SVGGElement, GraphNode>()
        .on('start', (event, d) => {
          if (!event.active) simulation.alphaTarget(0.3).restart();
          d.fx = d.x;
          d.fy = d.y;
        })
        .on('drag', (event, d) => {
          d.fx = event.x;
          d.fy = event.y;
        })
        .on('end', (event, d) => {
          if (!event.active) simulation.alphaTarget(0);
          d.fx = null;
          d.fy = null;
        }));

    // Add circles for nodes
    node.append('circle')
      .attr('r', 20)
      .attr('fill', d => statusColors[d.task.status as keyof typeof statusColors] || statusColors.todo)
      .attr('stroke', d => priorityColors[d.task.priority])
      .attr('stroke-width', 3)
      .attr('opacity', 0.8);

    // Add task ID labels
    node.append('text')
      .attr('text-anchor', 'middle')
      .attr('dy', '.3em')
      .attr('font-size', '12px')
      .attr('font-weight', 'bold')
      .attr('fill', 'white')
      .text(d => `#${d.task.id}`);

    // Add hover tooltips
    node.append('title')
      .text(d => `Task #${d.task.id}: ${d.task.description}\nStatus: ${d.task.status}\nPriority: ${d.task.priority}\nPhase: ${d.task.phase.name}`);

    // Add click handlers
    node.on('click', (event, d) => {
      event.stopPropagation();
      onTaskClick?.(d.task);
    });

    // Add task description labels (shown on hover)
    const labels = g.append('g')
      .selectAll('text')
      .data(nodes)
      .enter().append('text')
      .attr('text-anchor', 'middle')
      .attr('dy', '35px')
      .attr('font-size', '10px')
      .attr('fill', '#374151')
      .style('opacity', 0)
      .style('pointer-events', 'none')
      .text(d => d.task.description.length > 30 
        ? d.task.description.substring(0, 30) + '...'
        : d.task.description);

    // Show/hide labels on hover
    node
      .on('mouseenter', function(_, d) {
        d3.select(this).select('circle').attr('r', 25);
        labels.filter(label => label.id === d.id).style('opacity', 1);
      })
      .on('mouseleave', function(_, d) {
        d3.select(this).select('circle').attr('r', 20);
        labels.filter(label => label.id === d.id).style('opacity', 0);
      });

    // Update positions on simulation tick
    simulation.on('tick', () => {
      link
        .attr('x1', d => (d.source as GraphNode).x!)
        .attr('y1', d => (d.source as GraphNode).y!)
        .attr('x2', d => (d.target as GraphNode).x!)
        .attr('y2', d => (d.target as GraphNode).y!);

      node.attr('transform', d => `translate(${d.x},${d.y})`);
      labels.attr('transform', d => `translate(${d.x},${d.y})`);
    });

    // Store zoom behavior for external controls
    (svgRef.current as any).__zoom__ = zoomBehavior;

  }, [tasks, dimensions, onTaskClick]);

  const calculateTaskLevel = (task: Task, allTasks: Task[]): number => {
    const visited = new Set<number>();
    
    const dfs = (taskId: number): number => {
      if (visited.has(taskId)) return 0;
      visited.add(taskId);
      
      const currentTask = allTasks.find(t => t.id === taskId);
      if (!currentTask || currentTask.dependencies.length === 0) return 0;
      
      return 1 + Math.max(...currentTask.dependencies.map(dfs));
    };
    
    return dfs(task.id);
  };

  const handleZoomIn = () => {
    const svg = d3.select(svgRef.current!);
    const zoomBehavior = (svgRef.current as any).__zoom__;
    if (zoomBehavior) {
      svg.transition().call(zoomBehavior.scaleBy, 1.5);
    }
  };

  const handleZoomOut = () => {
    const svg = d3.select(svgRef.current!);
    const zoomBehavior = (svgRef.current as any).__zoom__;
    if (zoomBehavior) {
      svg.transition().call(zoomBehavior.scaleBy, 0.67);
    }
  };

  const handleReset = () => {
    const svg = d3.select(svgRef.current!);
    const zoomBehavior = (svgRef.current as any).__zoom__;
    if (zoomBehavior) {
      svg.transition().call(zoomBehavior.transform, d3.zoomIdentity);
    }
  };

  const handleFitToScreen = () => {
    const svg = d3.select(svgRef.current!);
    const zoomBehavior = (svgRef.current as any).__zoom__;
    if (zoomBehavior) {
      svg.transition().call(
        zoomBehavior.transform,
        d3.zoomIdentity.translate(dimensions.width / 2, dimensions.height / 2).scale(0.8)
      );
    }
  };

  return (
    <Card className={`${className}`} padding={false}>
      <Card.Header className="border-b border-gray-200 p-4">
        <div className="flex items-center justify-between">
          <Card.Title>Task Dependencies</Card.Title>
          <div className="flex items-center gap-2">
            <span className="text-sm text-gray-500">
              Zoom: {Math.round(zoom * 100)}%
            </span>
            <div className="flex gap-1">
              <Button size="sm" variant="ghost" onClick={handleZoomIn}>
                <ZoomIn className="h-4 w-4" />
              </Button>
              <Button size="sm" variant="ghost" onClick={handleZoomOut}>
                <ZoomOut className="h-4 w-4" />
              </Button>
              <Button size="sm" variant="ghost" onClick={handleReset}>
                <RotateCcw className="h-4 w-4" />
              </Button>
              <Button size="sm" variant="ghost" onClick={handleFitToScreen}>
                <Maximize2 className="h-4 w-4" />
              </Button>
            </div>
          </div>
        </div>
      </Card.Header>

      <div className="relative" ref={containerRef} style={{ height: '600px' }}>
        {tasks.length === 0 ? (
          <div className="flex items-center justify-center h-full text-gray-500">
            <div className="text-center">
              <p className="text-lg font-medium mb-2">No tasks available</p>
              <p className="text-sm">Add some tasks to see the dependency graph</p>
            </div>
          </div>
        ) : (
          <svg
            ref={svgRef}
            width={dimensions.width}
            height={dimensions.height}
            className="border-0"
          />
        )}
      </div>

      {/* Legend */}
      <div className="border-t border-gray-200 p-4">
        <div className="grid grid-cols-2 gap-4 text-xs">
          <div>
            <h4 className="font-medium text-gray-900 mb-2">Status</h4>
            <div className="space-y-1">
              {Object.entries(statusColors).map(([status, color]) => (
                <div key={status} className="flex items-center gap-2">
                  <div 
                    className="w-3 h-3 rounded-full"
                    style={{ backgroundColor: color }}
                  />
                  <span className="capitalize">{status.replace('-', ' ')}</span>
                </div>
              ))}
            </div>
          </div>
          <div>
            <h4 className="font-medium text-gray-900 mb-2">Priority (Border)</h4>
            <div className="space-y-1">
              {Object.entries(priorityColors).map(([priority, color]) => (
                <div key={priority} className="flex items-center gap-2">
                  <div 
                    className="w-3 h-3 rounded-full border-2"
                    style={{ borderColor: color, backgroundColor: 'transparent' }}
                  />
                  <span>{priority}</span>
                </div>
              ))}
            </div>
          </div>
        </div>
        <div className="mt-3 text-gray-600">
          <p>• Drag nodes to reposition • Click nodes for details • Arrows show dependencies</p>
        </div>
      </div>
    </Card>
  );
};

export default DependencyGraph;