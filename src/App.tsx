import React, { Fragment, ChangeEvent } from 'react';
import './App.css';
import { CssBaseline, CircularProgress } from '@material-ui/core';
import { wrap, transfer } from 'comlink';
import { DependencyGraph } from './components/DependencyGraph';
import { Sidebar } from './components/Sidebar';
import { ComponentType, Component, Link } from './types/Components';

const reader = new FileReader();
const worker = new Worker('./web-worker', { name: 'analyzer', type: 'module' });
const workerApi = wrap<import('./web-worker').Analyzer>(worker);

const colorMap = {
  'Text': '#b748d9',
  'Number': '#dddea2',
  'MultipleChoice': '#48d960',
  'TrueFalse': '#dea2a2',
  'Date': '#48bed9',
  'Dialog': '#d57652',
  'Computation': '#0c4160'
};

function App() {
  let [state, updateState] = React.useState({ loading: false, data: { nodes: new Array<Component>(), links: new Array<Link>() }, activeFilter: 'All' });

  const loadComponentFile = (e: ChangeEvent<HTMLInputElement>) => {
    reader.onload = async componentFile => {
      updateState({...state, loading: true, data: { nodes: [], links: []}});
      let data = componentFile.target?.result as ArrayBuffer;
      const [components, links] = await workerApi.analyze(transfer(data, [data]), colorMap);

      updateState({ ...state, data: { nodes: components, links}, loading: false });
    };
    reader.readAsArrayBuffer(e.target.files![0]);
  };

  const childElement = (() => {
    if (state.loading) {
      return <CircularProgress size={200}/>;
    }

    if (state.data.nodes.length > 0) {
      return <DependencyGraph data={state.data}></DependencyGraph>
    }
    return <div></div>;
  })();
  
  return (
      <Fragment>
        <CssBaseline />
        <Sidebar 
          activeFilter={state.activeFilter} 
          setComponentFilter={(componentType: ComponentType) => {
            let newNodes = state.data.nodes.map(node => ({ 
              ...node, 
              strokeColor: (componentType === 'All' || node.type === componentType) ? 'black' : 'white',
              fillColor: (componentType === 'All' || node.type === componentType) ? 'black' : 'white',
            }));
            updateState({ ...state, activeFilter: componentType, data: { nodes: newNodes, links: state.data.links }});
          }}
          colorMap={colorMap}></Sidebar>
        <main>
          <div>
            <input id='cmpLoader' type='file' onChange={loadComponentFile}></input>
          </div>
          {childElement}
        </main>
      </Fragment>
  );
}

export default App;
