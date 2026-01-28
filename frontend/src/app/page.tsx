'use client';

import { useState } from 'react';
import styles from './page.module.css';

interface AccountData {
  address: string;
  lamports: number;
  owner: string;
  executable: boolean;
  rent_epoch: number;
}

export default function Dashboard() {
  const [inputAddress, setInputAddress] = useState('');
  const [data, setData] = useState<AccountData[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchData = async () => {
    if (!inputAddress) return;
    setLoading(true);
    try {
      const addresses = inputAddress.split(',').map(s => s.trim()).filter(s => s);

      // Call backend
      const res = await fetch('http://127.0.0.1:9000/data', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ data: addresses }),
      });

      if (res.ok) {
        const jsonData = await res.json();
        setData(jsonData);
      } else {
        console.error('Failed to fetch from backend');
        alert('Failed to fetch data. Ensure backend is running.');
      }
    } catch (e) {
      console.error(e);
      alert('Error connecting to backend.');
    }
    setLoading(false);
  };

  return (
    <div className={styles.dashboard}>
      <header className={styles.header}>
        <div>
          <div className={styles.breadcrumb}>Dashboard / Solana Stats</div>
          <h1 className={styles.title}>Network Overview</h1>
        </div>

      </header>

      <div className={styles.statsGrid}>
        <div className={styles.card}>
          <div className={styles.cardContent}>
            <span className={styles.cardLabel}>Total Fetched</span>
            <h3 className={styles.cardValue}>{data.length}</h3>
            <span className={styles.cardSubtext}>Accounts</span>
          </div>
          <div className={styles.cardIcon}>📊</div>
        </div>
        <div className={styles.card}>
          <div className={styles.cardContent}>
            <span className={styles.cardLabel}>Total Balance</span>
            <h3 className={styles.cardValue}>
              {(data.reduce((acc, curr) => acc + curr.lamports, 0) / 1000000000).toFixed(2)}
            </h3>
            <span className={styles.cardSubtext}>SOL</span>
          </div>
          <div className={styles.cardIcon}>💰</div>
        </div>
      </div>

      <div className={styles.gridRow}>
        {/* Input Section */}
        <div className={`${styles.card} ${styles.inputSection}`}>
          <h3 className={styles.sectionTitle}>Add Data</h3>
          <p className={styles.sectionDesc}>Enter Solana addresses to fetch their live data from mainnet.</p>
          <div className={styles.inputGroup}>
            <input
              type="text"
              className={styles.input}
              placeholder="Enter Address (e.g. So111...)"
              value={inputAddress}
              onChange={e => setInputAddress(e.target.value)}
            />
            <button className={styles.button} onClick={fetchData} disabled={loading}>
              {loading ? 'Fetching...' : 'Add Data'}
            </button>
          </div>
        </div>

        {/* Chart/Visual Section */}
        <div className={`${styles.card} ${styles.visualSection}`}>
          <h3 className={styles.sectionTitle}>Data Visualization</h3>
          <div className={styles.barChartPlaceholder}>
            {data.map((d, i) => (
              <div key={i} className={styles.barContainer} title={`${d.address}: ${d.lamports} Lamports`}>
                <div
                  className={styles.bar}
                  style={{ height: `${Math.min((d.lamports / 1000000000) * 10, 100)}%` }} // Scaling for visual
                ></div>
                <span className={styles.barLabel}>{d.address.slice(0, 4)}</span>
              </div>
            ))}
            {data.length === 0 && <span className={styles.placeholderText}>No data to visualize</span>}
          </div>
        </div>
      </div>

      {/* Detailed Table */}
      <div className={`${styles.card} ${styles.tableSection}`}>
        <h3 className={styles.sectionTitle}>Live Account Data</h3>
        <div className={styles.tableWrapper}>
          <table className={styles.table}>
            <thead>
              <tr>
                <th>Address</th>
                <th>Balance (SOL)</th>
                <th>Owner</th>
                <th>Rent Epoch</th>
                <th>Executable</th>
              </tr>
            </thead>
            <tbody>
              {data.length > 0 ? data.map((item, idx) => (
                <tr key={idx}>
                  <td className={styles.addressCell} title={item.address}>{item.address.slice(0, 8)}...{item.address.slice(-8)}</td>
                  <td className={styles.boldCell}>{(item.lamports / 1000000000).toFixed(5)}</td>
                  <td className={styles.addressCell} title={item.owner}>{item.owner.slice(0, 8)}...</td>
                  <td>{item.rent_epoch}</td>
                  <td>
                    <span className={item.executable ? styles.badgeYes : styles.badgeNo}>
                      {item.executable ? 'YES' : 'NO'}
                    </span>
                  </td>
                </tr>
              )) : (
                <tr><td colSpan={5} style={{ textAlign: 'center', padding: '20px' }}>No data fetched yet</td></tr>
              )}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
}
