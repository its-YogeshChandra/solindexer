import { DM_Sans } from 'next/font/google';
import './globals.css';

const dmSans = DM_Sans({ subsets: ['latin'], weight: ['400', '500', '700'] });

export const metadata = {
  title: 'Flup Solana Dashboard',
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body className={dmSans.className} style={{ display: 'flex', flexDirection: 'column', backgroundColor: '#F4F7FE', minHeight: '100vh' }}>
        <main style={{ width: '100%', maxWidth: '1200px', margin: '0 auto', padding: '30px' }}>
          {children}
        </main>
      </body>
    </html>
  );
}
