export type ComponentType = "All" | "Text" | "Number" | "Date" | "TrueFalse" | "MultipleChoice" | "Image" | "SingleSelect" ;

export interface Component {
    id: string,
    labelPosition: string,
    color: string,
    symbolType: string,
    type: string,
    component: any
}

export interface Link {
    source: string,
    target: string
}