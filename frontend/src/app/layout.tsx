import type { Metadata } from 'next'
import './globals.css'

export const metadata: Metadata = {
  title: 'Stellar-OjaBridge',
  description: 'Stellar SEP-24 Anchor for African Markets — seamless fiat-to-crypto on/off-ramps.',
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  )
}
