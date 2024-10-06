import Head from 'next/head'
import { useState } from 'react'
import styles from '../styles/Home.module.css'

export default function Home() {
    const [copied, setCopied] = useState(false)
    const ca = '87wEGGtM8vrywhFNSitSLC78jmhWLCiPpWGqqGyQpump'

    const copyToClipboard = async () => {
        try {
            await navigator.clipboard.writeText(ca)
            setCopied(true)
            setTimeout(() => setCopied(false), 2000)
        } catch (err) {
            console.error('Failed to copy text: ', err)
        }
    }

    return (
        <div className={styles.container}>
            <Head>
                <title>Bitcoin Founder Mystery</title>
                <meta name="description" content="Discover the real Bitcoin founder" />
                <link rel="icon" href="/favicon.ico" />
            </Head>

            <main className={styles.main}>
                <h1 className={styles.title} onClick={copyToClipboard}>
                    CA: {ca}
                </h1>
                {copied && <p className={styles.copied}>Copied to clipboard!</p>}
                <p className={styles.description}>
                    You wanna find out who is the real bitcoin founder?<br />
                    Click to the CA and find it out
                </p>
                <img
                    src="https://pump.mypinata.cloud/ipfs/QmQWVNvHNVYj52LuV3UufZoc2eY9UUQ1FVrmDEyGEfm3Fi?img-width=256&img-dpr=2&img-onerror=redirect"
                    alt="Bitcoin Logo"
                    className={styles.logo}
                />
            </main>
        </div>
    )
}