program ntzreisers.aleo;

closure reisersTableLookup:
    input r0 as u32;
    is.eq r0 0u32 into r1;
    not r1 into r2;
    is.eq r0 1u32 into r3;
    and r2 r3 into r4;
    not r3 into r5;
    is.eq r0 2u32 into r6;
    and r2 r5 into r7;
    and r7 r6 into r8;
    not r6 into r9;
    is.eq r0 3u32 into r10;
    and r7 r9 into r11;
    and r11 r10 into r12;
    not r10 into r13;
    is.eq r0 4u32 into r14;
    and r11 r13 into r15;
    and r15 r14 into r16;
    not r14 into r17;
    is.eq r0 5u32 into r18;
    and r15 r17 into r19;
    and r19 r18 into r20;
    not r18 into r21;
    is.eq r0 6u32 into r22;
    and r19 r21 into r23;
    and r23 r22 into r24;
    not r22 into r25;
    is.eq r0 7u32 into r26;
    and r23 r25 into r27;
    and r27 r26 into r28;
    not r26 into r29;
    is.eq r0 8u32 into r30;
    and r27 r29 into r31;
    and r31 r30 into r32;
    not r30 into r33;
    is.eq r0 9u32 into r34;
    and r31 r33 into r35;
    and r35 r34 into r36;
    not r34 into r37;
    is.eq r0 10u32 into r38;
    and r35 r37 into r39;
    and r39 r38 into r40;
    not r38 into r41;
    is.eq r0 11u32 into r42;
    and r39 r41 into r43;
    and r43 r42 into r44;
    not r42 into r45;
    is.eq r0 12u32 into r46;
    and r43 r45 into r47;
    and r47 r46 into r48;
    not r46 into r49;
    is.eq r0 13u32 into r50;
    and r47 r49 into r51;
    and r51 r50 into r52;
    not r50 into r53;
    is.eq r0 14u32 into r54;
    and r51 r53 into r55;
    and r55 r54 into r56;
    not r54 into r57;
    is.eq r0 15u32 into r58;
    and r55 r57 into r59;
    and r59 r58 into r60;
    not r58 into r61;
    is.eq r0 16u32 into r62;
    and r59 r61 into r63;
    and r63 r62 into r64;
    not r62 into r65;
    is.eq r0 17u32 into r66;
    and r63 r65 into r67;
    and r67 r66 into r68;
    not r66 into r69;
    is.eq r0 18u32 into r70;
    and r67 r69 into r71;
    and r71 r70 into r72;
    not r70 into r73;
    is.eq r0 19u32 into r74;
    and r71 r73 into r75;
    and r75 r74 into r76;
    not r74 into r77;
    is.eq r0 20u32 into r78;
    and r75 r77 into r79;
    and r79 r78 into r80;
    not r78 into r81;
    is.eq r0 21u32 into r82;
    and r79 r81 into r83;
    and r83 r82 into r84;
    not r82 into r85;
    is.eq r0 22u32 into r86;
    and r83 r85 into r87;
    and r87 r86 into r88;
    not r86 into r89;
    is.eq r0 23u32 into r90;
    and r87 r89 into r91;
    and r91 r90 into r92;
    not r90 into r93;
    is.eq r0 24u32 into r94;
    and r91 r93 into r95;
    and r95 r94 into r96;
    not r94 into r97;
    is.eq r0 25u32 into r98;
    and r95 r97 into r99;
    and r99 r98 into r100;
    not r98 into r101;
    is.eq r0 26u32 into r102;
    and r99 r101 into r103;
    and r103 r102 into r104;
    not r102 into r105;
    is.eq r0 27u32 into r106;
    and r103 r105 into r107;
    and r107 r106 into r108;
    not r106 into r109;
    is.eq r0 28u32 into r110;
    and r107 r109 into r111;
    and r111 r110 into r112;
    not r110 into r113;
    is.eq r0 29u32 into r114;
    and r111 r113 into r115;
    and r115 r114 into r116;
    not r114 into r117;
    is.eq r0 30u32 into r118;
    and r115 r117 into r119;
    and r119 r118 into r120;
    not r118 into r121;
    is.eq r0 31u32 into r122;
    and r119 r121 into r123;
    and r123 r122 into r124;
    not r122 into r125;
    is.eq r0 32u32 into r126;
    and r123 r125 into r127;
    and r127 r126 into r128;
    not r126 into r129;
    is.eq r0 33u32 into r130;
    and r127 r129 into r131;
    and r131 r130 into r132;
    not r130 into r133;
    is.eq r0 34u32 into r134;
    and r131 r133 into r135;
    and r135 r134 into r136;
    not r134 into r137;
    is.eq r0 35u32 into r138;
    and r135 r137 into r139;
    and r139 r138 into r140;
    not r138 into r141;
    is.eq r0 36u32 into r142;
    and r139 r141 into r143;
    and r143 r142 into r144;
    not r142 into r145;
    and r143 r145 into r146;
    ternary r144 18u8 0u8 into r147;
    ternary r140 19u8 r147 into r148;
    ternary r136 8u8 r148 into r149;
    ternary r132 20u8 r149 into r150;
    ternary r128 5u8 r150 into r151;
    ternary r124 9u8 r151 into r152;
    ternary r120 14u8 r152 into r153;
    ternary r116 21u8 r153 into r154;
    ternary r112 0u8 r154 into r155;
    ternary r108 6u8 r155 into r156;
    ternary r104 12u8 r156 into r157;
    ternary r100 10u8 r157 into r158;
    ternary r96 29u8 r158 into r159;
    ternary r92 15u8 r159 into r160;
    ternary r88 31u8 r160 into r161;
    ternary r84 22u8 r161 into r162;
    ternary r80 25u8 r162 into r163;
    ternary r76 0u8 r163 into r164;
    ternary r72 17u8 r164 into r165;
    ternary r68 7u8 r165 into r166;
    ternary r64 4u8 r166 into r167;
    ternary r60 13u8 r167 into r168;
    ternary r56 0u8 r168 into r169;
    ternary r52 11u8 r169 into r170;
    ternary r48 28u8 r170 into r171;
    ternary r44 30u8 r171 into r172;
    ternary r40 24u8 r172 into r173;
    ternary r36 16u8 r173 into r174;
    ternary r32 3u8 r174 into r175;
    ternary r28 0u8 r175 into r176;
    ternary r24 27u8 r176 into r177;
    ternary r20 23u8 r177 into r178;
    ternary r16 2u8 r178 into r179;
    ternary r12 26u8 r179 into r180;
    ternary r8 1u8 r180 into r181;
    ternary r4 0u8 r181 into r182;
    ternary r1 32u8 r182 into r183;
    output r183 as u8;

function main:
    input r0 as u32.public;
    sub.w 0u32 r0 into r1;
    and r0 r1 into r2;
    rem.w r2 37u32 into r3;
    call reisersTableLookup r3 into r4;
    output r4 as u8.private;
