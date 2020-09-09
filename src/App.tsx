import React, { Fragment, ChangeEvent } from 'react';
import './App.css';
import { CssBaseline, CircularProgress } from '@material-ui/core';
import { wrap, transfer } from 'comlink';
import { DependencyGraph } from './components/DependencyGraph';
import { Sidebar } from './components/Sidebar';

const reader = new FileReader();
const worker = new Worker('./web-worker', { name: 'analyzer', type: 'module' });
const workerApi = wrap<import('./web-worker').Analyzer>(worker);

function App() {
  let [state, updateState] = React.useState({ loading: false, selectedNode: undefined, data: { nodes: [], links: []}, typeFilter: "All" });

  const loadComponentFile = (e: ChangeEvent<HTMLInputElement>) => {
    reader.onload = async componentFile => {
      updateState({...state, loading: true, data: { nodes: [], links: []}});
      let data = componentFile.target?.result as ArrayBuffer;
      const [components, links] = await workerApi.analyze(transfer(data, [data]));

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
        <Sidebar updateComponentFilter={() => {}}></Sidebar>
        <main>
          <div>
            <input id="cmpLoader" type="file" onChange={loadComponentFile}></input>
          </div>
          {childElement}
        </main>
      </Fragment>
  );
}

export default App;
