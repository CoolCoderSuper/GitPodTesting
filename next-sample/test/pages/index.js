import Head from 'next/head'
import styles from '../styles/Home.module.css'

export default function Home() {
  return (
    <div className={styles.container}>
      <Head>
        <title>Test CodeSpaces</title>
        <h1>This is a test from GitHub CodeSpaces</h1>
      </Head>
    </div>
  )
}
