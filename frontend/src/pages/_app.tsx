import '../styles/globals.css'
import {useState} from "react";
import {query, useQuery} from "../libs/dbConnection";
import {observer} from "mobx-react";

const MyApp = observer(() => {
    const [entryText, setEntryText] = useState("")
    const [entries, refreshEntries] = useQuery("SELECT content FROM entry", undefined, 1000)

    async function addEntry(text: string) {
        console.log("Adding entry")
        await query("CREATE entry SET content = $content", {
            "content": text
        })

        refreshEntries()
    }

    return (
        <div>
            <div className="flex flex-row space-x-4">
                <p className="text-lg">Hello World!</p>
                <div className="flex flex-col">
                    <input onChange={(e) => setEntryText(e.target.value)}/>
                    <button onClick={(e) => addEntry(entryText)}>Add the Entry</button>
                </div>
                <div className="flex flex-col">
                    {entries.length > 0 ? entries[0].result.map((item: any, index: number) => <p
                        key={index}>{item.content}</p>) : <></>}
                </div>
            </div>
        </div>
    )
})

export default MyApp
