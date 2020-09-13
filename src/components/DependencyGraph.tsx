import React from 'react';
import { Graph, GraphData, GraphNode, GraphLink } from 'react-d3-graph';

export function DependencyGraph(props: {data: GraphData<GraphNode, GraphLink>}) {
    return (
        <Graph
            id="graph-id"
            config={{
                directed: true,
                width: 1700,
                height: 1000,
                focusAnimationDuration: 1,
                nodeHighlightBehavior: true,
                highlightDegree: 1,
                node: {
                    strokeColor: 'black',
                    strokeWidth: 1
                },
                link: {
                    color: '#c9cdff'
                }
            }}
            data={props.data}
        >
        </Graph>
    );   
}