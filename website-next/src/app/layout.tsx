import type { Metadata, Viewport } from "next";
import { Inter, IBM_Plex_Mono, Syne } from "next/font/google";
import "./globals.css";
import { Toaster } from "@/components/ui/toaster";

const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
  weight: ["300", "400", "500", "600", "700"],
});

const ibmPlexMono = IBM_Plex_Mono({
  variable: "--font-mono",
  subsets: ["latin"],
  weight: ["400", "500", "600"],
});

const syne = Syne({
  variable: "--font-syne",
  subsets: ["latin"],
  weight: ["400", "600", "700", "800"],
});

export const metadata: Metadata = {
  title: "GhostWire — A Protocol for Resilient P2P Mesh Communication",
  description:
    "GhostWire is an open protocol for resilient peer-to-peer communication. A decentralized mesh networking stack built in Rust — encrypted, delay-tolerant, post-quantum ready, and AI-routed. No servers. No owners. Build communication tools that work regardless of the odds.",
  keywords: [
    "GhostWire",
    "P2P messaging",
    "resilient mesh",
    "end-to-end encrypted messaging",
    "decentralized messaging",
    "NAT traversal P2P",
    "E2EE messaging protocol",
    "offline-capable messaging",
    "mesh networking",
    "libp2p",
    "Rust",
    "censorship-resistant",
    "disaster response",
    "delay-tolerant networking",
    "post-quantum cryptography",
  ],
  authors: [{ name: "Team GhostWire" }],
  metadataBase: new URL("https://www.ghostwire.cc"),
  alternates: {
    canonical: "/",
    types: {
      "application/rss+xml": "https://www.ghostwire.cc/feed.xml",
    },
  },
  manifest: "/manifest.json",
  applicationName: "GhostWire",
  appleWebApp: {
    capable: true,
    statusBarStyle: "black-translucent",
    title: "GhostWire",
  },
  formatDetection: {
    telephone: false,
    address: false,
    email: false,
  },
  icons: {
    icon: [
      { url: "/favicon.ico", sizes: "any" },
      { url: "/ghostwire-logo.png", sizes: "32x32", type: "image/png" },
      { url: "/ghostwire-logo.png", sizes: "16x16", type: "image/png" },
    ],
    apple: [{ url: "/ghostwire-logo.png", sizes: "180x180", type: "image/png" }],
    shortcut: "/favicon.ico",
  },
  openGraph: {
    title: "GhostWire — A Protocol for Resilient P2P Mesh Communication",
    description:
      "An open protocol for resilient peer-to-peer communication. Encrypted, delay-tolerant, post-quantum ready, and AI-routed. No servers. No owners.",
    url: "https://www.ghostwire.cc/",
    siteName: "GhostWire",
    type: "website",
    locale: "en_US",
    images: [
      {
        url: "/og-image.png",
        width: 1200,
        height: 630,
        alt: "GhostWire — Resilient Mesh Communication Protocol",
      },
    ],
  },
  twitter: {
    card: "summary_large_image",
    title: "GhostWire — A Protocol for Resilient P2P Mesh Communication",
    description:
      "An open protocol for resilient peer-to-peer mesh communication. P2P, E2EE, AI-routed, delay-tolerant. No servers. No owners.",
    images: ["/og-image.png"],
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      "max-image-preview": "large",
    },
  },
  verification: {
    google: "jK-eYCRNuT9s4KLjGRp1z52JtTQY0uDRUiDd7_LSqLA",
  },
  category: "technology",
};

export const viewport: Viewport = {
  themeColor: "#070a0e",
  colorScheme: "dark",
  width: "device-width",
  initialScale: 1,
  maximumScale: 5,
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body
        className={`${inter.variable} ${ibmPlexMono.variable} ${syne.variable} antialiased`}
      >
        {children}
        <Toaster />
      </body>
    </html>
  );
}
