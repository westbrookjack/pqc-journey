pub fn gcd_triple(alpha: u64, beta: u64) -> (u64, i128, i128) {
    if alpha == 0 {
        (beta, 0i128, 1i128)
    }
    else if beta == 0 {
        (alpha, 1i128 , 0i128)
    }
    else {
        let a : i128;
        let b : i128;

        if alpha<beta {
            a = beta as i128;
            b = alpha as i128;
        } else {
            a = alpha as i128;
            b = beta as i128;
        }
        
        let mut q : i128 = a/b;
        let mut tup1:(i128, i128, i128) = (a,1,0);
        let mut tup2:(i128, i128, i128) = (b,0,1);
        let mut holder:(i128, i128, i128);

        

        while q*tup2.0 != tup1.0 {
            holder = tup1;
            tup1 = tup2;
            tup2.0 = holder.0%tup1.0;
            tup2.1 = holder.1-q*tup1.1;
            tup2.2 = holder.2-q*tup1.2;
            q = tup1.0/tup2.0;
        }

        if alpha< beta {
            let c:i128 = tup2.1;
            tup2.1 = tup2.2;
            tup2.2 = c;
        }

        assert!(tup2.0==tup2.1*i128::from(alpha)+tup2.2*i128::from(beta), "Something went wrong in the computation.");

        (u64::try_from(tup2.0).expect("Value out of u64 range"), tup2.1, tup2.2)
        }

        
}
