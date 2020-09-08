import React, { Fragment, ChangeEvent } from 'react';
import './App.css';

import { Drawer, List, ListItem, ListItemIcon, ListItemText, CssBaseline, CircularProgress } from '@material-ui/core';
import { Inbox, TextFields, CalendarToday, DoneOutline, FormatListBulleted } from '@material-ui/icons';
import { Graph } from 'react-d3-graph';
import { wrap, transfer } from 'comlink';

enum ComponentType {
  All,
  Text,
  Number,
  Date,
  TrueFalse,
  MultipleChoice
}
const r = new FileReader();

const worker = new Worker('./web-worker', { name: 'analyzer', type: 'module' });
const workerApi = wrap<import('./web-worker').Analyzer>(worker);

function App() {
  let [state, updateState] = React.useState({ loading: false, selectedNode: undefined, data: { nodes: [], links: []}, typeFilter: ComponentType.All });

  const loadComponentFile = (e: ChangeEvent<HTMLInputElement>) => {
    r.onload = async x => {
      updateState({...state, loading: true, data: { nodes: [], links: []}});
      let data = x.target?.result as ArrayBuffer;
      const [components, links] = await workerApi.analyze(transfer(data, [data]));

      updateState({ ...state, data: { nodes: components, links}, loading: false });
    };
    r.readAsArrayBuffer(e.target.files![0]);
  };

  return (
      <Fragment>
        <CssBaseline />
        <Drawer variant="permanent" style={{width: "220px" }}>
          <List>
              <ListItem button onClick={() => updateState({...state, typeFilter: ComponentType.All})}>
                <ListItemIcon><Inbox /></ListItemIcon>
                <ListItemText primary="All Components" />
              </ListItem>
              <ListItem button onClick={() => updateState({...state, typeFilter: ComponentType.Text})}>
                <ListItemIcon><TextFields /></ListItemIcon>
                <ListItemText primary="Text" />
              </ListItem>
              <ListItem button onClick={() => updateState({...state, typeFilter: ComponentType.Number})}>
                <ListItemIcon><Inbox /></ListItemIcon>
                <ListItemText primary="Number" />
              </ListItem>
              <ListItem button onClick={() => updateState({...state, typeFilter: ComponentType.Date})}>
                <ListItemIcon><CalendarToday /></ListItemIcon>
                <ListItemText primary="Date" />
              </ListItem>
              <ListItem button onClick={() => updateState({...state, typeFilter: ComponentType.TrueFalse})}>
                <ListItemIcon><DoneOutline /></ListItemIcon>
                <ListItemText primary="True\False" />
              </ListItem>
              <ListItem button onClick={() => updateState({...state, typeFilter: ComponentType.MultipleChoice})}>
                <ListItemIcon><FormatListBulleted /></ListItemIcon>
                <ListItemText primary="Multiple Choice" />
              </ListItem>
            </List>
        </Drawer>
        <main>
            <input id="cmpLoader" type="file" onChange={loadComponentFile}></input>
            
            { state.data.nodes.length > 0
            ?
            <Graph
              id="graph-id"
              config={{
                directed: true,
                width: 1700,
                height: 1000,
                focusAnimationDuration: 1
              }}
              data={state.data}
            >
            </Graph>
            : <div></div>
            }

          { state.loading ? <CircularProgress size={200}/> : null }
        </main>
        
      </Fragment>
  );
}

export default App;
