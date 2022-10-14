import '../styles/globals.css'
import {useStore} from "../stores";
import {useEffect, useState} from "react";

const MyApp = () => {
    const root = useStore()

    const [entryText, setEntryText] = useState("")
    const [entries, setEntries] = useState<string[]>([])


    async function syncEntries() {
        const response = await root.dbStore.query("SELECT content FROM entry")
        const entries = response[0].result
        setEntries(entries.map((item: any) => item.content))
    }

    async function addEntry(text: string) {
        await root.dbStore.query("CREATE entry SET content = $content", {
            "content": text
        })

        await syncEntries()
    }

    useEffect(() => {
        syncEntries()
    }, [])

    return (
        <div>
            <div className="flex flex-row">
                <p className="text-lg">Hello World!</p>
                <div className="flex flex-col">
                    <input onChange={(e) => setEntryText(e.target.value)}/>
                    <button onClick={(e) => addEntry(entryText)}>Add the Entry</button>
                </div>
                <div className="flex flex-col">
                    {entries.map((item, index) => <p key={index}>{item}</p>)}
                </div>
            </div>
        </div>
    )
}

export default MyApp
