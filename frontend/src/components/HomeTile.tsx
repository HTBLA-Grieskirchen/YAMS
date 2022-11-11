import {observer} from "mobx-react";
import {ReactNode} from "react";

const HomeTile = observer((
    {icon, title, children}:
        { icon: string, title: string, children?: ReactNode }
) => {
    return <div className="group flex flex-col place-items-center space-y-4 p-0 m-1 transition-all rounded-md shadow-sm hover:shadow-md bg-white
        border border-blue-600 hover:bg-blue-600 hover:p-1 hover:m-0">
        <div className="flex flex-col mx-4 mt-8 w-24 h-24 text-center bg-blue-600 rounded-full text-white
            group-hover:bg-gray-800">
            <i className={`text-6xl my-auto fa-solid ${icon}`}></i>
        </div>
        <div className="mx-4">
            <p className="text-2xl font-semibold tracking-tight text-blue-600 group-hover:text-white">{title}</p>
        </div>
        <div className="pb-4 px-2 text-center group-hover:text-gray-200 break-words leading-snug">
            {children}
        </div>
    </div>
})

export default HomeTile