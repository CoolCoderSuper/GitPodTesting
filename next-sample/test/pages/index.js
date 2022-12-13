import Head from 'next/head'
import styles from '../styles/Home.module.css'
let notes = [{id: 1, value: "hello"}, {id: 2, value: "bye"}]
let notesItems = notes.map((note) => <div key={note.id}><input value={note.value}/><button>X</button></div>)
export default function Home() {
  return (
    <div className={styles.container}>
      <Head>
        <title>Notes</title>
        <div>{notesItems}</div>
      </Head>
    </div>
  )
}
