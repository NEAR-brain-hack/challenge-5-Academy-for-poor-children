import 'regenerator-runtime/runtime'
import React from 'react'
import { login, logout } from './utils'
import './global.css'
import educationOnPoverty from './assets/educationOnPoverty.jpg'
import multilingualEducation from './assets/Multilingual-Education.jpg'

import getConfig from './config'
import { async } from 'regenerator-runtime/runtime'
const { networkId } = getConfig(process.env.NODE_ENV || 'development')
import { utils, transactions } from "near-api-js";

const NFT_MINT_GAS = '0.003';

export default function App() {
  const [nfts, setNfts] = React.useState(0)

  const donate = async () => {
    const result = await window.account.signAndSendTransaction({
      receiverId: window.contract.contractId,
      actions: [
          transactions.functionCall(
              'nft_mint', 
              {
                receiver_id: window.accountId
              },
              10000000000000, 
              utils.format.parseNearAmount("2")
          )
      ]
    });
  }

  React.useEffect(
    () => {
      if (window.walletConnection.isSignedIn()) {
        window?.contract?.nft_supply_for_owner({account_id: window.accountId}).then(result => {
          setNfts(result)
          console.log(result)
        })
      }
    },
    []
  )

  // if not signed in, return early with sign-in prompt
  if (!window.walletConnection.isSignedIn()) {
    return (
      <main>
        <h1>NEAR HACKS ACADEMY</h1>
        <div className='study-on-poverty-img'>
          <img src={educationOnPoverty} />
        </div>
        <p>
          393 MILLION CHILDREN UNABLE TO READ <br />
          More than 393 million children have failed to gain the basic literacy skills at age 10 since world leaders adopted the Sustainable Development Goals in 2015, according to a new analysis tool launched today. <br />
          <i>
            <a href="https://www.savethechildren.net/news/393-million-children-unable-read-world%E2%80%99s-shocking-lost-potential#:~:text=393%20million%20children%20unable%20to%20read%3A%20The%20world's%20shocking%20lost%20potential,-A%20new%20analysis&text=More%20than%20393%20million%20children,new%20analysis%20tool%20launched%20today.">
              (www.savethechildren.net)
            </a>
          </i>
        </p>
        <p>
          For each NFT you mint on this app, you could help more children to gain education:
        </p>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
          <button onClick={login}>Sign in</button>
        </p>
      </main>
    )
  }

  return (
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    <>
      <button className="link" style={{ float: 'right' }} onClick={logout}>
        Sign out
      </button>
      <main>
        <div className='study-on-poverty-img'>
          <img src={multilingualEducation} />
        </div>
        <p>
          Donate for poor children by minting NFT. Two NAER would be sent to them for each NFT minted.
        </p>
        <p style={{ textAlign: 'center', marginTop: '2.5em' }}>
          <h2>(Thank you for mint {nfts} for poor children)</h2>
          <h3>please check your wallet to see them</h3>
        </p>
        <button onClick={donate}>Donate more</button>
          <br />
      </main>
    </>
  )
}