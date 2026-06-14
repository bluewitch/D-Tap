```mermaid
flowchart TD
    %% Point of Sale
    subgraph POS[Point of Sale]
        A[QR/NFC Scanner] -->|Payment Request| B[D-Tap API Endpoint]
    end

    %% D-Tap Service
    subgraph SVC ["D-Tap Service"]
        B --> C[Validate Request]
        C --> D[Fetch Quote (DEX Aggregator)]
        D --> E[Build Swap Tx (Alloy)]
        E --> F[Sign Tx (Alloy Wallet)]
        F --> G[Send Tx to Blockchain]
        G --> H[Wait Confirmation]
        H --> I[Swap Output -> Fresh Address]
        I --> J[Transfer to Gift Card Contract]
        J --> K[Mint/Transfer Gift Card to User]
        K --> L[Return Receipt + Gift Card QR]
    end

    %% External Services
    subgraph EXT [External Services]
        D -->|Quote| Dex[DEX Aggregator (1inch, Paraswap)]
        G -->|HyperLiquid API| HL[HyperLiquid REST/WS]
        G -->|Polymarket API| PM[Polymarket REST]
        J -->|Gift Card Contract| GC[ERC-1155/ERC-721 Gift Card]
        I -->|Fresh Address| FA[Newly Generated Address (Clean Coins)]
    end

    %% Alloy Wallet
    subgraph WLT [Alloy Wallet]
        F -->|Sign| WLT
        WLT -->|Private Key| Key[Secure Key Store (Env/HSM)]
    end

    %% Styling
    classDef pos fill:#f9f,stroke:#333,stroke-width:2px;
    classDef svc fill:#bbf,stroke:#333,stroke-width:2px;
    classDef ext fill:#bfb,stroke:#333,stroke-width:2px;
    classDef wlt fill:#ff9,stroke:#333,stroke-width:2px;
    class POS pos;
    class SVC svc;
    class EXT ext;
    class WLT wlt;
```
