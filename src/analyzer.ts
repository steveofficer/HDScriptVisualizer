const wasmLoader = import('./native/build');

function getString(content: string | ArrayBuffer): string {
    if (content instanceof ArrayBuffer) {
        const decoder = new TextDecoder("utf-8");
        return decoder.decode(content);
    }

    return content;
}

export async function analyze(content: string | ArrayBuffer | null | undefined): Promise<any> {
    if (content === undefined || content === null) {
        return Promise.resolve([null, null]);
    }

    try {
        const wasm = await wasmLoader;
        const visualElements = wasm.analyze(getString(content));
        const names = Object.keys(visualElements);

        let components = names.map((name: string) => {
            const component = visualElements[name];
            let color = "purple";

            if (typeof component === "string") {
                switch (component) {
                    case 'Text': color = "#ffa600"; break;
                    case 'Number': color = "#374c80"; break;
                    case 'MultipleChoice': color = "#7a5195"; break;
                    case 'TrueFalse': color = "#bc5090"; break;
                    case 'Date': color = "#ef5675"; break;
                }
              } else if (component.Dialog) {
                color = "#ff764a"
              } else {
                color = "#003f5c";
              }

            return {
                id: name,
                labelPosition: 'bottom',
                color,
                component
            };
        });

        let links = components
            .filter(c => typeof c.component === "object")
            .flatMap(c => {
                if (c.component.Dialog) {
                    let children = c.component.Dialog.children.map((childName: string) => ({ source: childName, target: c.id }));
                    let script = c.component.Dialog.script.map((childName: string) => ({ source: childName, target: c.id }));
                    return [...children, ...script];
                } else {
                    return c.component.Computation.map((childName: string) => ({ source: childName, target: c.id }));
                }
            })
            .filter(c => names.indexOf(c.source) !== -1 && names.indexOf(c.target) !== -1);
        return [components, links];
    }
    catch (e) {
        console.error(`${e}`);
    }
}

/*
if (typeof t === "string") {
    key = t;
    switch (t) {
        case 'Text': color = "#ffa600"; break;
        case 'Number': color = "#374c80"; break;
        case 'MultipleChoice': color = "#7a5195"; break;
        case 'TrueFalse': color = "#bc5090"; break;
        case 'Date': color = "#ef5675"; break;
    }
  } else if (t.Dialog) {
    color = "#ff764a"
  } else {
    color = "#003f5c";
  }
  */