
class CoinsWidget extends HTMLElement {
    constructor() {
        super();

        this.coins = [];

        this.evtSource = new EventSource("http://127.0.0.1:8000/sse/?symbols=Bitcoin", {
        });
        this.evtSource.onmessage = (event) => {
            const data = JSON.parse(event.data)
            console.log(event.data);

            //store coins
            this.coins = data;
            localStorage.setItem("coins", this.coins);
        };
    }

    connectedCallback() {
        this.coins = localStorage.getItem("coins");
        //this.fetchData();

        // Add scroll event listener
        //window.addEventListener("scroll", this.handleScroll.bind(this));

    }

    disconnectedCallback() {
        // Remove scroll event listener when the component is removed from the DOM
        //window.removeEventListener("scroll", this.handleScroll.bind(this));
    }
}

customElements.define("coins-widget", CoinsWidget);