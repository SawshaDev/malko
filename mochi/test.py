import aiohttp

import asyncio

async def main():
    session = aiohttp.ClientSession()

    ws = await session.ws_connect("ws://127.0.0.1:1200")
    await ws.send_json({"type": "PING"})

    while True:
        msg = await ws.receive()
        print(f"{msg.json()}")

asyncio.run(main())