```mermaid
flowchart TD
    %% Point of Sale
    subgraph POS[Point of Sale]
        A[QR/NFC Scanner] -->|Payment Request| B[D-Tap API Endpoint]
    end

    %% D-Tap Service
    subgraph SVC ["D-Tap Service"]
        B --> C[Validate Request]
        C -->|Fetch Quote DEX Aggregator| D[DEX Aggregator]
        D --> E[Get Quote]
        E -->|Build Swap Tx Alloy| F[Build Swap Tx]
        F -->|Sign Tx Alloy Wallet| G[Sign Tx]
        G --> H[Send Tx to Blockchain]
        H --> I[Wait Confirmation]
        I -->|Swap Output -> Fresh Address Clean Coins| J[Fresh Address]
        J --> K[Transfer to Gift Card Contract]
        K --> L[Mint/Transfer Gift Card to User]
        L --> M[Return Receipt Gift Card QR]
    end

    %% External Services
    subgraph EXT [External Services]
        D --> Dex[DEX Aggregator 1inch Paraswap]
        G --> HL[HyperLiquid API]
        G --> PM[Polymarket API]
        K --> GC[Gift Card Contract ERC1155 ERC721]
        J --> FA[Fresh Address]
    end

    %% Alloy Wallet
    subgraph WLT [Alloy Wallet]
        F -->|Sign| WLT
        WLT --> Key[Secure Key Store Env/HSM]
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
