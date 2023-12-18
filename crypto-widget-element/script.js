const baseURL = "http://localhost:8000";
class CoinInfo extends HTMLElement {
    constructor() {
        super();
        this.coinInfo = {};
    }

    render() {
        this.container = document.createElement("div");
        this.container.className = "coin-info";

        this.name = document.createElement("h1");
        this.name.className = "coin-name";
        this.name.textContent = this.coinInfo.symbol;
        this.container.appendChild(this.name);
        this.logo = document.createElement("img");
        this.logo.className = "coin-logo";
        this.logo.src = baseURL + "/public/" + this.coinInfo.id + ".png";
        this.container.appendChild(this.logo);

        this.price = document.createElement("a");
        this.price.className = "coin-price";
        this.price.textContent = this.coinInfo.price;
        this.container.appendChild(this.price);

        this.marketcap = document.createElement("a");
        this.marketcap.className = "coin-marketcap";
        this.marketcap.textContent = this.coinInfo.marketcap;
        this.container.appendChild(this.marketcap);

        this.appendChild(this.container);
    }

    connectedCallback() {
        this.render(); // Move the child appending logic here
    }
}
class CoinsWidget extends HTMLElement {
    constructor() {
        super();
        this.coins = "Bitcoin,Kaspa";
        this.results = new Map();

        this.headerTitle = document.createElement("h1");
        this.headerTitle.className = "coin-widget-title";
        this.headerTitle.textContent = "Coin Ticker";
        this.appendChild(this.headerTitle);


        this.coinContainer = document.createElement("div");
        this.coinContainer.className = "coin-container";
        this.appendChild(this.coinContainer);

        this.evtSource = new EventSource(baseURL + "/sse/?symbols=" + this.coins);

        this.evtSource.onmessage = (event) => {
            const data = JSON.parse(event.data);

            for (let key in data) {
                if (data.hasOwnProperty(key)) {
                    if (this.results.has(key)) {
                        const coinElement = this.results.get(key);
                        coinElement.coinInfo = data[key];
                    } else {
                        const coin = document.createElement("coin-info");
                        coin.coinInfo = data[key];
                        this.results.set(key, coin);
                        this.coinContainer.appendChild(coin);
                    }
                }
            }

            localStorage.setItem("coins", this.coins);
        };
    }

    connectedCallback() {
        this.coins = localStorage.getItem("coins");
        // this.fetchData();
        // window.addEventListener("scroll", this.handleScroll.bind(this));
    }

    disconnectedCallback() {
        // window.removeEventListener("scroll", this.handleScroll.bind(this));
    }
}

customElements.define("coin-info", CoinInfo);
customElements.define("coins-widget", CoinsWidget);
