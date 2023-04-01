open System
open Notes
printfn "Notes App"
let mutable cont = true
while cont do
    Console.Write("> ")
    let cmd = Console.ReadLine()
    cont = not(App.run cmd)