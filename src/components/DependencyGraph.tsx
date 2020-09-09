import React from 'react';
import { Graph } from 'react-d3-graph';

export function DependencyGraph(props: {data: any}) {
    return (
        <Graph
            id="graph-id"
            config={{
                directed: true,
                width: 1700,
                height: 1000,
                focusAnimationDuration: 1,
                nodeHighlightBehavior: true,
                highlightDegree: 1
            }}
            data={props.data}
        >
        </Graph>
    );   
}