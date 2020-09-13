import { Component, Link } from './types/Components';

const wasmLoader = import('./native/build');

function getString(content: string | ArrayBuffer): string {
    if (content instanceof ArrayBuffer) {
        const decoder = new TextDecoder('utf-8');
        return decoder.decode(content);
    }

    return content;
}

export async function analyze(content: string | ArrayBuffer | null | undefined, typeColorMap: { [K:string]: string }): Promise<[Component[], Link[]]> {
    if (content === undefined || content === null) {
        return [[], []];
    }

    try {
        const wasm = await wasmLoader;
        const visualElements = wasm.analyze(getString(content));
        const names = Object.keys(visualElements);

        let components = names.map((name: string) => {
            const component = visualElements[name];
            let node = {
                id: name,
                labelPosition: 'bottom',
                color: 'purple',
                symbolType: 'circle',
                component,
                type: ''
            };
            
            if (typeof component === 'string') {
                node.type = component;
                node.color = typeColorMap[node.type];
            } else if (component.Dialog) {
                node.type = 'Dialog';
                node.symbolType = 'square';
            } else {
                node.type = 'Computation';
                node.symbolType = 'triangle';
            }
            node.color = typeColorMap[node.type];
            
            return node;
        });

        let links = components
            .filter(c => typeof c.component === 'object')
            .flatMap(c => {
                if (c.component.Dialog) {
                    const referencedNames = new Set<string>([...c.component.Dialog.children, ...c.component.Dialog.script]);
                    let children = [];
                    for (const name of referencedNames) {
                        children.push({ source: name, target: c.id });
                    }
                    return children;
                } else {
                    return c.component.Computation.map((childName: string) => ({ source: childName, target: c.id }));
                }
            })
            .filter(c => names.indexOf(c.source) !== -1 && names.indexOf(c.target) !== -1);
        return [components, links];
    }
    catch (e) {
        console.error(`${e}`);
        return [[], []];
    }
}
