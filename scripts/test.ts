import "dotenv/config"
const MY_TOKEN_METADATA = {
    name: "TEST",
    symbol: "TEST",
    description: "This is a test token!",
    image: "https://URL_TO_YOUR_IMAGE.png" //add public URL to image you'd like to use
}

const ON_CHAIN_METADATA = {
    name: MY_TOKEN_METADATA.name, 
    symbol: MY_TOKEN_METADATA.symbol,
    uri: 'TO_UPDATE_LATER',
    sellerFeeBasisPoints: 0,
    creators: null,
    collection: null,
    uses: null
} 

function main() {
    console.log(process.env.ADMIN_KEY)
}

main()