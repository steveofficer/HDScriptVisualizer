import React from 'react';
import { Drawer, List, ListItem, ListItemIcon, ListItemText } from '@material-ui/core';
import { Inbox, TextFields, CalendarToday, DoneOutline, FormatListBulleted } from '@material-ui/icons';
import { ComponentType } from '../types/Components';

export function Sidebar(props: { activeFilter: string, setComponentFilter(component: ComponentType): void }) {
    return (
        <Drawer variant="permanent" style={{width: "220px" }}>
          <List>
              <ListItem button selected={props.activeFilter === "All"} onClick={() => props.setComponentFilter("All")}>
                <ListItemIcon><Inbox /></ListItemIcon>
                <ListItemText primary="All Components" />
              </ListItem>
              <ListItem button selected={props.activeFilter === "Text"} onClick={() => props.setComponentFilter("Text")}>
                <ListItemIcon><TextFields /></ListItemIcon>
                <ListItemText primary="Text" />
              </ListItem>
              <ListItem button selected={props.activeFilter === "Number"} onClick={() => props.setComponentFilter("Number")}>
                <ListItemIcon><Inbox /></ListItemIcon>
                <ListItemText primary="Number" />
              </ListItem>
              <ListItem button selected={props.activeFilter === "Date"} onClick={() => props.setComponentFilter("Date")}>
                <ListItemIcon><CalendarToday /></ListItemIcon>
                <ListItemText primary="Date" />
              </ListItem>
              <ListItem button selected={props.activeFilter === "TrueFalse"} onClick={() => props.setComponentFilter("TrueFalse")}>
                <ListItemIcon><DoneOutline /></ListItemIcon>
                <ListItemText primary="True\False" />
              </ListItem>
              <ListItem button selected={props.activeFilter === "MultipleChoice"} onClick={() => props.setComponentFilter("MultipleChoice")}>
                <ListItemIcon><FormatListBulleted /></ListItemIcon>
                <ListItemText primary="Multiple Choice" />
              </ListItem>
            </List>
        </Drawer>
    );
}