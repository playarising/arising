// SPDX-License-Identifier: MIT
pragma solidity 0.8.16;

contract LevelsTable {
    function getLevel(uint256 exp) public pure returns (uint256) {
        if (exp < 1000) {
            return 0;
        } else if (exp >= 1000 && exp < 2020) {
            return 1;
        } else if (exp >= 2020 && exp < 3060) {
            return 2;
        } else if (exp >= 3060 && exp < 4122) {
            return 3;
        } else if (exp >= 4122 && exp < 5204) {
            return 4;
        } else if (exp >= 5204 && exp < 6321) {
            return 5;
        } else if (exp >= 6321 && exp < 7474) {
            return 6;
        } else if (exp >= 7474 && exp < 8664) {
            return 7;
        } else if (exp >= 8664 && exp < 9891) {
            return 8;
        } else if (exp >= 9891 && exp < 11158) {
            return 9;
        } else if (exp >= 11158 && exp < 12466) {
            return 10;
        } else if (exp >= 12466 && exp < 13816) {
            return 11;
        } else if (exp >= 13816 && exp < 15208) {
            return 12;
        } else if (exp >= 15208 && exp < 16645) {
            return 13;
        } else if (exp >= 16645 && exp < 18133) {
            return 14;
        } else if (exp >= 18133 && exp < 19668) {
            return 15;
        } else if (exp >= 19668 && exp < 21252) {
            return 16;
        } else if (exp >= 21252 && exp < 22887) {
            return 17;
        } else if (exp >= 22887 && exp < 24574) {
            return 18;
        } else if (exp >= 24574 && exp < 26316) {
            return 19;
        } else if (exp >= 26316 && exp < 28092) {
            return 20;
        } else if (exp >= 28092 && exp < 29903) {
            return 21;
        } else if (exp >= 29903 && exp < 31751) {
            return 22;
        } else if (exp >= 31751 && exp < 33636) {
            return 23;
        } else if (exp >= 33636 && exp < 35558) {
            return 24;
        } else if (exp >= 35558 && exp < 37519) {
            return 25;
        } else if (exp >= 37519 && exp < 39519) {
            return 26;
        } else if (exp >= 39519 && exp < 41559) {
            return 27;
        } else if (exp >= 41559 && exp < 43640) {
            return 28;
        } else if (exp >= 43640 && exp < 45763) {
            return 29;
        } else if (exp >= 45763 && exp < 47928) {
            return 30;
        } else if (exp >= 47928 && exp < 50136) {
            return 31;
        } else if (exp >= 50136 && exp < 52389) {
            return 32;
        } else if (exp >= 52389 && exp < 54686) {
            return 33;
        } else if (exp >= 54686 && exp < 57064) {
            return 34;
        } else if (exp >= 57064 && exp < 59525) {
            return 35;
        } else if (exp >= 59525 && exp < 62073) {
            return 36;
        } else if (exp >= 62073 && exp < 64709) {
            return 37;
        } else if (exp >= 64709 && exp < 67438) {
            return 38;
        } else if (exp >= 67438 && exp < 70262) {
            return 39;
        } else if (exp >= 70262 && exp < 73177) {
            return 40;
        } else if (exp >= 73177 && exp < 76185) {
            return 41;
        } else if (exp >= 76185 && exp < 79289) {
            return 42;
        } else if (exp >= 79289 && exp < 82492) {
            return 43;
        } else if (exp >= 82492 && exp < 85798) {
            return 44;
        } else if (exp >= 85798 && exp < 89170) {
            return 45;
        } else if (exp >= 89170 && exp < 92610) {
            return 46;
        } else if (exp >= 92610 && exp < 96118) {
            return 47;
        } else if (exp >= 96118 && exp < 99697) {
            return 48;
        } else if (exp >= 99697 && exp < 103347) {
            return 49;
        } else if (exp >= 103347 && exp < 107114) {
            return 50;
        } else if (exp >= 107114 && exp < 111001) {
            return 51;
        } else if (exp >= 111001 && exp < 115013) {
            return 52;
        } else if (exp >= 115013 && exp < 119153) {
            return 53;
        } else if (exp >= 119153 && exp < 123426) {
            return 54;
        } else if (exp >= 123426 && exp < 127784) {
            return 55;
        } else if (exp >= 127784 && exp < 132229) {
            return 56;
        } else if (exp >= 132229 && exp < 136763) {
            return 57;
        } else if (exp >= 136763 && exp < 141388) {
            return 58;
        } else if (exp >= 141388 && exp < 146105) {
            return 59;
        } else if (exp >= 146105 && exp < 150974) {
            return 60;
        } else if (exp >= 150974 && exp < 155998) {
            return 61;
        } else if (exp >= 155998 && exp < 161183) {
            return 62;
        } else if (exp >= 161183 && exp < 166534) {
            return 63;
        } else if (exp >= 166534 && exp < 172056) {
            return 64;
        } else if (exp >= 172056 && exp < 177754) {
            return 65;
        } else if (exp >= 177754 && exp < 183635) {
            return 66;
        } else if (exp >= 183635 && exp < 189705) {
            return 67;
        } else if (exp >= 189705 && exp < 195968) {
            return 68;
        } else if (exp >= 195968 && exp < 202432) {
            return 69;
        } else if (exp >= 202432 && exp < 209122) {
            return 70;
        } else if (exp >= 209122 && exp < 216047) {
            return 71;
        } else if (exp >= 216047 && exp < 223213) {
            return 72;
        } else if (exp >= 223213 && exp < 230631) {
            return 73;
        } else if (exp >= 230631 && exp < 238308) {
            return 74;
        } else if (exp >= 238308 && exp < 246254) {
            return 75;
        } else if (exp >= 246254 && exp < 254478) {
            return 76;
        } else if (exp >= 254478 && exp < 262989) {
            return 77;
        } else if (exp >= 262989 && exp < 271799) {
            return 78;
        } else if (exp >= 271799 && exp < 280917) {
            return 79;
        } else if (exp >= 280917 && exp < 290327) {
            return 80;
        } else if (exp >= 290327 && exp < 300038) {
            return 81;
        } else if (exp >= 300038 && exp < 310060) {
            return 82;
        } else if (exp >= 310060 && exp < 320402) {
            return 83;
        } else if (exp >= 320402 && exp < 331075) {
            return 84;
        } else if (exp >= 331075 && exp < 341962) {
            return 85;
        } else if (exp >= 341962 && exp < 353066) {
            return 86;
        } else if (exp >= 353066 && exp < 364393) {
            return 87;
        } else if (exp >= 364393 && exp < 375946) {
            return 88;
        } else if (exp >= 375946 && exp < 387730) {
            return 89;
        } else if (exp >= 387730 && exp < 399892) {
            return 90;
        } else if (exp >= 399892 && exp < 412442) {
            return 91;
        } else if (exp >= 412442 && exp < 425394) {
            return 92;
        } else if (exp >= 425394 && exp < 438761) {
            return 93;
        } else if (exp >= 438761 && exp < 452555) {
            return 94;
        } else if (exp >= 452555 && exp < 466791) {
            return 95;
        } else if (exp >= 466791 && exp < 481482) {
            return 96;
        } else if (exp >= 481482 && exp < 496643) {
            return 97;
        } else if (exp >= 496643 && exp < 512290) {
            return 98;
        } else if (exp >= 512290 && exp < 528437) {
            return 99;
        } else if (exp >= 528437 && exp < 544907) {
            return 100;
        } else if (exp >= 544907 && exp < 561707) {
            return 101;
        } else if (exp >= 561707 && exp < 578842) {
            return 102;
        } else if (exp >= 578842 && exp < 596320) {
            return 103;
        } else if (exp >= 596320 && exp < 614148) {
            return 104;
        } else if (exp >= 614148 && exp < 632332) {
            return 105;
        } else if (exp >= 632332 && exp < 650880) {
            return 106;
        } else if (exp >= 650880 && exp < 669799) {
            return 107;
        } else if (exp >= 669799 && exp < 689097) {
            return 108;
        } else if (exp >= 689097 && exp < 708780) {
            return 109;
        } else if (exp >= 708780 && exp < 729152) {
            return 110;
        } else if (exp >= 729152 && exp < 750238) {
            return 111;
        } else if (exp >= 750238 && exp < 772061) {
            return 112;
        } else if (exp >= 772061 && exp < 794648) {
            return 113;
        } else if (exp >= 794648 && exp < 818026) {
            return 114;
        } else if (exp >= 818026 && exp < 842221) {
            return 115;
        } else if (exp >= 842221 && exp < 867264) {
            return 116;
        } else if (exp >= 867264 && exp < 893183) {
            return 117;
        } else if (exp >= 893183 && exp < 920010) {
            return 118;
        } else if (exp >= 920010 && exp < 947775) {
            return 119;
        } else if (exp >= 947775 && exp < 976429) {
            return 120;
        } else if (exp >= 976429 && exp < 1005999) {
            return 121;
        } else if (exp >= 1005999 && exp < 1036516) {
            return 122;
        } else if (exp >= 1036516 && exp < 1068010) {
            return 123;
        } else if (exp >= 1068010 && exp < 1100511) {
            return 124;
        } else if (exp >= 1100511 && exp < 1133662) {
            return 125;
        } else if (exp >= 1133662 && exp < 1167477) {
            return 126;
        } else if (exp >= 1167477 && exp < 1201967) {
            return 127;
        } else if (exp >= 1201967 && exp < 1237148) {
            return 128;
        } else if (exp >= 1237148 && exp < 1273032) {
            return 129;
        } else if (exp >= 1273032 && exp < 1309634) {
            return 130;
        } else if (exp >= 1309634 && exp < 1346967) {
            return 131;
        } else if (exp >= 1346967 && exp < 1385048) {
            return 132;
        } else if (exp >= 1385048 && exp < 1423890) {
            return 133;
        } else if (exp >= 1423890 && exp < 1463509) {
            return 134;
        } else if (exp >= 1463509 && exp < 1504395) {
            return 135;
        } else if (exp >= 1504395 && exp < 1546590) {
            return 136;
        } else if (exp >= 1546590 && exp < 1590136) {
            return 137;
        } else if (exp >= 1590136 && exp < 1635074) {
            return 138;
        } else if (exp >= 1635074 && exp < 1681586) {
            return 139;
        } else if (exp >= 1681586 && exp < 1729726) {
            return 140;
        } else if (exp >= 1729726 && exp < 1779550) {
            return 141;
        } else if (exp >= 1779550 && exp < 1831118) {
            return 142;
        } else if (exp >= 1831118 && exp < 1884491) {
            return 143;
        } else if (exp >= 1884491 && exp < 1939733) {
            return 144;
        } else if (exp >= 1939733 && exp < 1996907) {
            return 145;
        } else if (exp >= 1996907 && exp < 2056083) {
            return 146;
        } else if (exp >= 2056083 && exp < 2117330) {
            return 147;
        } else if (exp >= 2117330 && exp < 2180720) {
            return 148;
        } else if (exp >= 2180720 && exp < 2246330) {
            return 149;
        } else {
            return 150;
        }
    }
}
