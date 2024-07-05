
init:
	lui	sp, 16
	addi	sp, sp, -80
	sw	ra, 0(sp)
	sw	s0, 4(sp)
	addi	s0, sp, 80
	addi	gp, s0, 0
	jal	ra, main
L_0_0:
	jal	x0, L_0_0

merge:
	addi	sp, sp, -128
	sw	ra, 0(sp)
	sw	s0, 4(sp)
	addi	s0, sp, 128
	sw	a0, -76(s0)
	addi	t0, s0, -76
	sw	a1, -80(s0)
	addi	t0, s0, -80
	sw	a2, -84(s0)
	addi	t0, s0, -84
	sub	t0, a2, a1
	addi	t0, t0, 1
	sw	t0, -88(s0)
	addi	t1, s0, -88
	sub	t1, a3, a2
	sw	t1, -92(s0)
	addi	t2, s0, -92
	slli	t0, t0, 2
	addi	t0, t0, 15
	andi	t0, t0, -16
	sub	sp, sp, t0
	addi	t2, sp, 0
	sw	t2, -104(s0)
	addi	t2, s0, -104
	slli	t1, t1, 2
	addi	t1, t1, 15
	andi	t1, t1, -16
	sub	sp, sp, t1
	addi	t2, sp, 0
	sw	t2, -108(s0)
	addi	t2, s0, -108
L_1_1:
	addi	t2, x0, 0
	sw	t2, -96(s0)
L_1_2:
	lw	t3, -96(s0)
	lw	t4, -88(s0)
	bge	t3, t4, L_1_4
	lw	t4, -104(s0)
	slli	t5, t3, 2
	add	t4, t4, t5
	lw	t5, -76(s0)
	lw	t6, -80(s0)
	add	t3, t6, t3
	slli	t3, t3, 2
	add	t3, t5, t3
	lw	t3, 0(t3)
	sw	t3, 0(t4)
L_1_3:
	lw	t3, -96(s0)
	addi	t3, t3, 1
	sw	t3, -96(s0)
	jal	x0, L_1_2
L_1_4:
L_1_5:
	sw	t2, -100(s0)
L_1_6:
	lw	t3, -100(s0)
	lw	t4, -92(s0)
	bge	t3, t4, L_1_8
	lw	t4, -108(s0)
	slli	t5, t3, 2
	add	t4, t4, t5
	lw	t5, -76(s0)
	lw	t6, -84(s0)
	addi	t6, t6, 1
	add	t3, t6, t3
	slli	t3, t3, 2
	add	t3, t5, t3
	lw	t3, 0(t3)
	sw	t3, 0(t4)
L_1_7:
	lw	t3, -100(s0)
	addi	t3, t3, 1
	sw	t3, -100(s0)
	jal	x0, L_1_6
L_1_8:
	sw	t2, -112(s0)
	addi	t3, s0, -112
	sw	t2, -116(s0)
	addi	t2, s0, -116
	lw	t2, -80(s0)
	sw	t2, -120(s0)
	addi	t2, s0, -120
L_1_9:
	lw	t2, -112(s0)
	lw	t3, -88(s0)
	sltu	t3, t2, t3
	lw	t4, -116(s0)
	lw	t5, -92(s0)
	sltu	t5, t4, t5
	and	t3, t3, t5
	beq	t3, x0, L_1_10
	lw	t3, -104(s0)
	slli	t2, t2, 2
	add	t5, t3, t2
	lw	t5, 0(t5)
	lw	t6, -108(s0)
	slli	t4, t4, 2
	add	t4, t6, t4
	lw	t4, 0(t4)
	blt	t4, t5, L_1_12
	lw	t4, -76(s0)
	lw	t5, -120(s0)
	slli	t5, t5, 2
	add	t4, t4, t5
	add	t2, t3, t2
	lw	t2, 0(t2)
	sw	t2, 0(t4)
	lw	t2, -112(s0)
	addi	t2, t2, 1
	sw	t2, -112(s0)
	jal	x0, L_1_11
L_1_12:
	lw	t2, -76(s0)
	lw	t3, -120(s0)
	slli	t3, t3, 2
	add	t2, t2, t3
	lw	t3, -108(s0)
	lw	t4, -116(s0)
	slli	t4, t4, 2
	add	t3, t3, t4
	lw	t3, 0(t3)
	sw	t3, 0(t2)
	lw	t2, -116(s0)
	addi	t2, t2, 1
	sw	t2, -116(s0)
L_1_11:
	lw	t2, -120(s0)
	addi	t2, t2, 1
	sw	t2, -120(s0)
	jal	x0, L_1_9
L_1_10:
L_1_13:
	lw	t2, -112(s0)
	lw	t3, -88(s0)
	bge	t2, t3, L_1_14
	lw	t3, -76(s0)
	lw	t4, -120(s0)
	slli	t4, t4, 2
	add	t3, t3, t4
	lw	t4, -104(s0)
	slli	t2, t2, 2
	add	t2, t4, t2
	lw	t2, 0(t2)
	sw	t2, 0(t3)
	lw	t2, -112(s0)
	addi	t2, t2, 1
	sw	t2, -112(s0)
	lw	t2, -120(s0)
	addi	t2, t2, 1
	sw	t2, -120(s0)
	jal	x0, L_1_13
L_1_14:
L_1_15:
	lw	t2, -116(s0)
	lw	t3, -92(s0)
	bge	t2, t3, L_1_16
	lw	t3, -76(s0)
	lw	t4, -120(s0)
	slli	t4, t4, 2
	add	t3, t3, t4
	lw	t4, -108(s0)
	slli	t2, t2, 2
	add	t2, t4, t2
	lw	t2, 0(t2)
	sw	t2, 0(t3)
	lw	t2, -116(s0)
	addi	t2, t2, 1
	sw	t2, -116(s0)
	lw	t2, -120(s0)
	addi	t2, t2, 1
	sw	t2, -120(s0)
	jal	x0, L_1_15
L_1_16:
	jal	x0, L_1_0
L_1_0:
	add	sp, sp, t1
	add	sp, sp, t0
	lw	ra, 0(sp)
	lw	s0, 4(sp)
	addi	sp, sp, 128
	jalr	x0, ra, 0

mergeSort:
	addi	sp, sp, -96
	sw	ra, 0(sp)
	sw	s0, 4(sp)
	addi	s0, sp, 96
	sw	a0, -76(s0)
	addi	t0, s0, -76
	sw	a1, -80(s0)
	addi	t0, s0, -80
	sw	a2, -84(s0)
	addi	t0, s0, -84
	blt	a1, a2, L_2_2
	jal	x0, L_2_0
L_2_2:
	sub	t0, a2, a1
	srli	t0, t0, 1
	add	t0, a1, t0
	sw	t0, -88(s0)
	addi	t1, s0, -88
	addi	a0, a0, 0
	addi	a1, a1, 0
	addi	a2, t0, 0
	jal	ra, mergeSort
	lw	t0, -76(s0)
	lw	t1, -88(s0)
	addi	t1, t1, 1
	lw	t2, -84(s0)
	addi	a0, t0, 0
	addi	a1, t1, 0
	addi	a2, t2, 0
	jal	ra, mergeSort
	lw	t0, -76(s0)
	lw	t1, -80(s0)
	lw	t2, -88(s0)
	lw	t3, -84(s0)
	addi	a0, t0, 0
	addi	a1, t1, 0
	addi	a2, t2, 0
	addi	a3, t3, 0
	jal	ra, merge
	jal	x0, L_2_0
L_2_0:
	lw	ra, 0(sp)
	lw	s0, 4(sp)
	addi	sp, sp, 96
	jalr	x0, ra, 0

main:
	addi	sp, sp, -96
	sw	ra, 0(sp)
	sw	s0, 4(sp)
	addi	s0, sp, 96
	addi	t0, x0, 10
	sw	t0, -76(s0)
	addi	t1, s0, -76
	slli	t0, t0, 2
	addi	t0, t0, 15
	andi	t0, t0, -16
	sub	sp, sp, t0
	addi	t1, sp, 0
	sw	t1, -80(s0)
	addi	t1, s0, -80
L_3_2:
	addi	t1, x0, 0
	sw	t1, -84(s0)
L_3_3:
	lw	t2, -84(s0)
	lw	t3, -76(s0)
	bge	t2, t3, L_3_5
	lw	t4, -80(s0)
	sub	t3, t3, t2
	addi	t5, x0, 1
	sub	t3, t3, t5
	addi	t5, x0, 2
	slli	t3, t3, 2
	add	t3, t4, t3
	sll	t2, t5, t2
	sw	t2, 0(t3)
L_3_4:
	lw	t2, -84(s0)
	addi	t2, t2, 1
	sw	t2, -84(s0)
	jal	x0, L_3_3
L_3_5:
	lw	t2, -80(s0)
	lw	t3, -76(s0)
	addi	t4, x0, 1
	sub	t3, t3, t4
	addi	a0, t2, 0
	addi	a1, t1, 0
	addi	a2, t3, 0
	sw	t0, -4(s0)
	sw	t1, -8(s0)
	jal	ra, mergeSort
	lw	t0, -4(s0)
	lw	t1, -8(s0)
	addi	a0, t1, 0
	jal	x0, L_3_0
L_3_0:
	add	sp, sp, t0
	lw	ra, 0(sp)
	lw	s0, 4(sp)
	addi	sp, sp, 96
	jalr	x0, ra, 0
