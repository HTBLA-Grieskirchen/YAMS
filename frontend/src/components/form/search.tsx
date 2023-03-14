import {observer} from "mobx-react";
import React from "react";

export const SmallSearchField = observer((
    {value, onChange}:
        { value: string, onChange: (value: string) => void }
) => {
    return <div className="form-control">
        <div className="input-group input-group-sm">
            <input type="text" placeholder="Search term..."
                   className="input input-bordered input-sm input-primary"
                   value={value} onChange={e => onChange(e.target.value)}/>
            {!!value.length ?
                <button className="btn btn-sm px-2 border-x-0" onClick={e => onChange("")}>
                    <i className="text-lg fa-solid fa-xmark w-4"/>
                </button> :
                <span className="px-2">
                    <i className="fa-solid fa-magnifying-glass w-4"/>
                </span>}
        </div>
    </div>
})
