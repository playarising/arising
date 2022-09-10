// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

import "../interfaces/ILevels.sol";

/**
 * @dev `Levels` is a basic tool to convert between experience points to level.
 */
contract Levels is ILevels {
    // =============================================== Getters ========================================================

    /**
     * @dev Returns the level from an experience amount.
     * @param exp   Experience amount.
     */
    function getLevel(uint256 exp) public pure returns (uint256) {
        if (exp < 1000) {
            return 0;
        } else if (exp >= 1000 && exp < 2020) {
            return 1;
        } else if (exp >= 2020 && exp < 3060) {
            return 2;
        } else if (exp >= 3060 && exp < 4121) {
            return 3;
        } else if (exp >= 4121 && exp < 5203) {
            return 4;
        } else if (exp >= 5203 && exp < 6320) {
            return 5;
        } else if (exp >= 6320 && exp < 7473) {
            return 6;
        } else if (exp >= 7473 && exp < 8663) {
            return 7;
        } else if (exp >= 8663 && exp < 9891) {
            return 8;
        } else if (exp >= 9891 && exp < 11158) {
            return 9;
        } else if (exp >= 11158 && exp < 12466) {
            return 10;
        } else if (exp >= 12466 && exp < 13816) {
            return 11;
        } else if (exp >= 13816 && exp < 15209) {
            return 12;
        } else if (exp >= 15209 && exp < 16647) {
            return 13;
        } else if (exp >= 16647 && exp < 18135) {
            return 14;
        } else if (exp >= 18135 && exp < 19671) {
            return 15;
        } else if (exp >= 19671 && exp < 21256) {
            return 16;
        } else if (exp >= 21256 && exp < 22892) {
            return 17;
        } else if (exp >= 22892 && exp < 24580) {
            return 18;
        } else if (exp >= 24580 && exp < 26322) {
            return 19;
        } else if (exp >= 26322 && exp < 28099) {
            return 20;
        } else if (exp >= 28099 && exp < 29912) {
            return 21;
        } else if (exp >= 29912 && exp < 31761) {
            return 22;
        } else if (exp >= 31761 && exp < 33647) {
            return 23;
        } else if (exp >= 33647 && exp < 35571) {
            return 24;
        } else if (exp >= 35571 && exp < 37533) {
            return 25;
        } else if (exp >= 37533 && exp < 39534) {
            return 26;
        } else if (exp >= 39534 && exp < 41575) {
            return 27;
        } else if (exp >= 41575 && exp < 43657) {
            return 28;
        } else if (exp >= 43657 && exp < 45781) {
            return 29;
        } else if (exp >= 45781 && exp < 47947) {
            return 30;
        } else if (exp >= 47947 && exp < 50156) {
            return 31;
        } else if (exp >= 50156 && exp < 52409) {
            return 32;
        } else if (exp >= 52409 && exp < 54707) {
            return 33;
        } else if (exp >= 54707 && exp < 57085) {
            return 34;
        } else if (exp >= 57085 && exp < 59546) {
            return 35;
        } else if (exp >= 59546 && exp < 62093) {
            return 36;
        } else if (exp >= 62093 && exp < 64729) {
            return 37;
        } else if (exp >= 64729 && exp < 67457) {
            return 38;
        } else if (exp >= 67457 && exp < 70280) {
            return 39;
        } else if (exp >= 70280 && exp < 73193) {
            return 40;
        } else if (exp >= 73193 && exp < 76199) {
            return 41;
        } else if (exp >= 76199 && exp < 79301) {
            return 42;
        } else if (exp >= 79301 && exp < 82502) {
            return 43;
        } else if (exp >= 82502 && exp < 85805) {
            return 44;
        } else if (exp >= 85805 && exp < 89174) {
            return 45;
        } else if (exp >= 89174 && exp < 92610) {
            return 46;
        } else if (exp >= 92610 && exp < 96115) {
            return 47;
        } else if (exp >= 96115 && exp < 99690) {
            return 48;
        } else if (exp >= 99690 && exp < 103337) {
            return 49;
        } else if (exp >= 103337 && exp < 107101) {
            return 50;
        } else if (exp >= 107101 && exp < 110985) {
            return 51;
        } else if (exp >= 110985 && exp < 114993) {
            return 52;
        } else if (exp >= 114993 && exp < 119129) {
            return 53;
        } else if (exp >= 119129 && exp < 123397) {
            return 54;
        } else if (exp >= 123397 && exp < 127750) {
            return 55;
        } else if (exp >= 127750 && exp < 132190) {
            return 56;
        } else if (exp >= 132190 && exp < 136719) {
            return 57;
        } else if (exp >= 136719 && exp < 141339) {
            return 58;
        } else if (exp >= 141339 && exp < 146051) {
            return 59;
        } else if (exp >= 146051 && exp < 150914) {
            return 60;
        } else if (exp >= 150914 && exp < 155933) {
            return 61;
        } else if (exp >= 155933 && exp < 161113) {
            return 62;
        } else if (exp >= 161113 && exp < 166459) {
            return 63;
        } else if (exp >= 166459 && exp < 171976) {
            return 64;
        } else if (exp >= 171976 && exp < 177670) {
            return 65;
        } else if (exp >= 177670 && exp < 183546) {
            return 66;
        } else if (exp >= 183546 && exp < 189610) {
            return 67;
        } else if (exp >= 189610 && exp < 195868) {
            return 68;
        } else if (exp >= 195868 && exp < 202326) {
            return 69;
        } else if (exp >= 202326 && exp < 209010) {
            return 70;
        } else if (exp >= 209010 && exp < 215928) {
            return 71;
        } else if (exp >= 215928 && exp < 223088) {
            return 72;
        } else if (exp >= 223088 && exp < 230499) {
            return 73;
        } else if (exp >= 230499 && exp < 238169) {
            return 74;
        } else if (exp >= 238169 && exp < 246107) {
            return 75;
        } else if (exp >= 246107 && exp < 254323) {
            return 76;
        } else if (exp >= 254323 && exp < 262827) {
            return 77;
        } else if (exp >= 262827 && exp < 271629) {
            return 78;
        } else if (exp >= 271629 && exp < 280739) {
            return 79;
        } else if (exp >= 280739 && exp < 290141) {
            return 80;
        } else if (exp >= 290141 && exp < 299844) {
            return 81;
        } else if (exp >= 299844 && exp < 309857) {
            return 82;
        } else if (exp >= 309857 && exp < 320190) {
            return 83;
        } else if (exp >= 320190 && exp < 330854) {
            return 84;
        } else if (exp >= 330854 && exp < 341731) {
            return 85;
        } else if (exp >= 341731 && exp < 352826) {
            return 86;
        } else if (exp >= 352826 && exp < 364143) {
            return 87;
        } else if (exp >= 364143 && exp < 375686) {
            return 88;
        } else if (exp >= 375686 && exp < 387460) {
            return 89;
        } else if (exp >= 387460 && exp < 399611) {
            return 90;
        } else if (exp >= 399611 && exp < 412151) {
            return 91;
        } else if (exp >= 412151 && exp < 425092) {
            return 92;
        } else if (exp >= 425092 && exp < 438447) {
            return 93;
        } else if (exp >= 438447 && exp < 452229) {
            return 94;
        } else if (exp >= 452229 && exp < 466452) {
            return 95;
        } else if (exp >= 466452 && exp < 481130) {
            return 96;
        } else if (exp >= 481130 && exp < 496278) {
            return 97;
        } else if (exp >= 496278 && exp < 511911) {
            return 98;
        } else if (exp >= 511911 && exp < 528044) {
            return 99;
        } else if (exp >= 528044 && exp < 544500) {
            return 100;
        } else if (exp >= 544500 && exp < 561285) {
            return 101;
        } else if (exp >= 561285 && exp < 578406) {
            return 102;
        } else if (exp >= 578406 && exp < 595869) {
            return 103;
        } else if (exp >= 595869 && exp < 613681) {
            return 104;
        } else if (exp >= 613681 && exp < 631849) {
            return 105;
        } else if (exp >= 631849 && exp < 650380) {
            return 106;
        } else if (exp >= 650380 && exp < 669282) {
            return 107;
        } else if (exp >= 669282 && exp < 688562) {
            return 108;
        } else if (exp >= 688562 && exp < 708228) {
            return 109;
        } else if (exp >= 708228 && exp < 728582) {
            return 110;
        } else if (exp >= 728582 && exp < 749648) {
            return 111;
        } else if (exp >= 749648 && exp < 771451) {
            return 112;
        } else if (exp >= 771451 && exp < 794017) {
            return 113;
        } else if (exp >= 794017 && exp < 817373) {
            return 114;
        } else if (exp >= 817373 && exp < 841546) {
            return 115;
        } else if (exp >= 841546 && exp < 866565) {
            return 116;
        } else if (exp >= 866565 && exp < 892460) {
            return 117;
        } else if (exp >= 892460 && exp < 919261) {
            return 118;
        } else if (exp >= 919261 && exp < 947000) {
            return 119;
        } else if (exp >= 947000 && exp < 975627) {
            return 120;
        } else if (exp >= 975627 && exp < 1005170) {
            return 121;
        } else if (exp >= 1005170 && exp < 1035658) {
            return 122;
        } else if (exp >= 1035658 && exp < 1067122) {
            return 123;
        } else if (exp >= 1067122 && exp < 1099593) {
            return 124;
        } else if (exp >= 1099593 && exp < 1132713) {
            return 125;
        } else if (exp >= 1132713 && exp < 1166495) {
            return 126;
        } else if (exp >= 1166495 && exp < 1200953) {
            return 127;
        } else if (exp >= 1200953 && exp < 1236100) {
            return 128;
        } else if (exp >= 1236100 && exp < 1271950) {
            return 129;
        } else if (exp >= 1271950 && exp < 1308517) {
            return 130;
        } else if (exp >= 1308517 && exp < 1345815) {
            return 131;
        } else if (exp >= 1345815 && exp < 1383859) {
            return 132;
        } else if (exp >= 1383859 && exp < 1422664) {
            return 133;
        } else if (exp >= 1422664 && exp < 1462245) {
            return 134;
        } else if (exp >= 1462245 && exp < 1503093) {
            return 135;
        } else if (exp >= 1503093 && exp < 1545248) {
            return 136;
        } else if (exp >= 1545248 && exp < 1588752) {
            return 137;
        } else if (exp >= 1588752 && exp < 1633648) {
            return 138;
        } else if (exp >= 1633648 && exp < 1680115) {
            return 139;
        } else if (exp >= 1680115 && exp < 1728208) {
            return 140;
        } else if (exp >= 1728208 && exp < 1777984) {
            return 141;
        } else if (exp >= 1777984 && exp < 1829502) {
            return 142;
        } else if (exp >= 1829502 && exp < 1882823) {
            return 143;
        } else if (exp >= 1882823 && exp < 1938010) {
            return 144;
        } else if (exp >= 1938010 && exp < 1995129) {
            return 145;
        } else if (exp >= 1995129 && exp < 2054247) {
            return 146;
        } else if (exp >= 2054247 && exp < 2115434) {
            return 147;
        } else if (exp >= 2115434 && exp < 2178763) {
            return 148;
        } else if (exp >= 2178763 && exp < 2244309) {
            return 149;
        } else {
            return 150;
        }
    }
}
