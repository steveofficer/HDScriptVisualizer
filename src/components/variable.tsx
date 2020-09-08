import React from 'react';

function Variable(props: { name: string, type: string }) {
    return(
        <div>
            <h5>{props.name}</h5>
            <span>{props.type}</span>
        </div>
    );
}

export default Variable;