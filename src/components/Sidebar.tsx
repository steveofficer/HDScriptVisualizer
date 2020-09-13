import React from 'react';
import { Drawer, List, ListItem, ListItemIcon, ListItemText } from '@material-ui/core';
import { Inbox, TextFields, CalendarToday, DoneOutline, FormatListBulleted } from '@material-ui/icons';
import { ComponentType } from '../types/Components';

export function Sidebar(props: { activeFilter: string, setComponentFilter(component: ComponentType): void, colorMap: { [x:string]: string } }) {
    return (
        <Drawer variant="permanent" style={{width: "220px" }}>
          <List>
            {
              [ 
                { name: "All", icon: <Inbox /> },
                { name: "Text", icon: <TextFields /> },
                { name: "Number", icon: <Inbox /> },
                { name: "Date", icon: <CalendarToday /> },
                { name: "TrueFalse", icon: <DoneOutline /> },
                { name: "MultipleChoice", icon: <FormatListBulleted /> },
              ].map(item => 
                <ListItem key={item.name} button selected={props.activeFilter === item.name} onClick={() => props.setComponentFilter(item.name as ComponentType)}>
                  <ListItemIcon style={{ color: props.colorMap[item.name] }}>{item.icon}</ListItemIcon>
                  <ListItemText primary={item.name} />
                </ListItem>
              )
            }
          </List>
        </Drawer>
    );
}