// Copyright © 2017-2021 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.
//
// This is a GENERATED FILE, changes made here MAY BE LOST.
// Generated one-time (codegen/bin/cointests)
//

#include "../interface/TWTestUtilities.h"
#include <TrustWalletCore/TWCoinTypeConfiguration.h>
#include <gtest/gtest.h>


TEST(TWFlowCoinType, TWCoinType) {
    auto symbol = WRAPS(TWCoinTypeConfigurationGetSymbol(TWCoinTypeFlow));
    auto txId = WRAPS(TWStringCreateWithUTF8Bytes("t123"));
    auto txUrl = WRAPS(TWCoinTypeConfigurationGetTransactionURL(TWCoinTypeFlow, txId.get()));
    auto accId = WRAPS(TWStringCreateWithUTF8Bytes("a12"));
    auto accUrl = WRAPS(TWCoinTypeConfigurationGetAccountURL(TWCoinTypeFlow, accId.get()));
    auto id = WRAPS(TWCoinTypeConfigurationGetID(TWCoinTypeFlow));
    auto name = WRAPS(TWCoinTypeConfigurationGetName(TWCoinTypeFlow));

    ASSERT_EQ(TWCoinTypeConfigurationGetDecimals(TWCoinTypeFlow), 8);
    ASSERT_EQ(TWBlockchainFlow, TWCoinTypeBlockchain(TWCoinTypeFlow));
    ASSERT_EQ(0x0, TWCoinTypeP2shPrefix(TWCoinTypeFlow));
    ASSERT_EQ(0x0, TWCoinTypeStaticPrefix(TWCoinTypeFlow));
    assertStringsEqual(symbol, "flow");
    assertStringsEqual(txUrl, "https://flowscan.org/transaction/t123");
    assertStringsEqual(accUrl, "https://flowscan.org/account/a12");
    assertStringsEqual(id, "flow");
    assertStringsEqual(name, "Flow");
}
