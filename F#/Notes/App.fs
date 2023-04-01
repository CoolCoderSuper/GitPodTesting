namespace Notes

module App = 

    let run(cmd: string) = 
        if cmd = "help" then
            printfn "add: creates a new note"
            printfn "remove: removes a note"
            printfn "list: lists all the notes"
            printfn "exit: closes the program"
            printfn "help: brings up this menu"
            false
        elif cmd = "add" then
            printfn "add"
            false
        elif cmd = "remove" then
            printfn "remove"
            false
        elif cmd = "list" then
            printfn "list"
            false
        elif cmd = "exit" then
            true
        else
            printfn "Invalid Command"
            false