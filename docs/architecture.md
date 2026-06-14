```mermaid
flowchart TD
    %% Point of Sale
    subgraph POS[Point of Sale]
        A[QR/NFC Scanner] -->|Payment Request| B[D-Tap API Endpoint]
    end

    %% D-Tap Service
    subgraph SVC ["D-Tap Service"]
        B --> C[Validate Request]
        C --> D|Fetch Quote [DEX Aggregator]| E[DEX Aggregator]
        E --> F[Get Quote]
        F --> G[Build Swap Tx [Alloy]]
        G --> H[Sign Tx [Alloy Wallet]]
        H --> I[Send Tx to Blockchain]
        I --> J[Wait Confirmation]
        J --> K[Swap Output -> Fresh Address [Clean Coins]]
        K --> L[Transfer to Gift Card Contract]
        L --> M[Mint/Transfer Gift Card to User]
        M --> N[Return Receipt + Gift Card QR]
    end

    %% External Services
    subgraph EXT [External Services]
        E --> Dex[DEX Aggregator [1inch, Paraswap]]
        G --> HL[HyperLiquid API]
        G --> PM[Polymarket API]
        L --> GC[Gift Card Contract [ERC-1155/ERC-721]]
        K --> FA[Fresh Address]
    end

    %% Alloy Wallet
    subgraph WLT [Alloy Wallet]
        H -->|Sign| WLT
        WLT --> Key[Secure Key Store (Env/HSM)]
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
