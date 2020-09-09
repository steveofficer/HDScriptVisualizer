import React from 'react';
import { Drawer, List, ListItem, ListItemIcon, ListItemText } from '@material-ui/core';
import { Inbox, TextFields, CalendarToday, DoneOutline, FormatListBulleted } from '@material-ui/icons';


export function Sidebar(props: { updateComponentFilter(component: string): void }) {
    return (
        <Drawer variant="permanent" style={{width: "220px" }}>
          <List>
              <ListItem button onClick={() => props.updateComponentFilter("All")}>
                <ListItemIcon><Inbox /></ListItemIcon>
                <ListItemText primary="All Components" />
              </ListItem>
              <ListItem button onClick={() => props.updateComponentFilter("Text")}>
                <ListItemIcon><TextFields /></ListItemIcon>
                <ListItemText primary="Text" />
              </ListItem>
              <ListItem button onClick={() => props.updateComponentFilter("Number")}>
                <ListItemIcon><Inbox /></ListItemIcon>
                <ListItemText primary="Number" />
              </ListItem>
              <ListItem button onClick={() => props.updateComponentFilter("Date")}>
                <ListItemIcon><CalendarToday /></ListItemIcon>
                <ListItemText primary="Date" />
              </ListItem>
              <ListItem button onClick={() => props.updateComponentFilter("TrueFalse")}>
                <ListItemIcon><DoneOutline /></ListItemIcon>
                <ListItemText primary="True\False" />
              </ListItem>
              <ListItem button onClick={() => props.updateComponentFilter("MultipleChoice")}>
                <ListItemIcon><FormatListBulleted /></ListItemIcon>
                <ListItemText primary="Multiple Choice" />
              </ListItem>
            </List>
        </Drawer>
    );
}