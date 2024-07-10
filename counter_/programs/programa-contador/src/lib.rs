use anchor_lang::prelude::*;

declare_id!("BK6ccqk4dr1L9sWhUt4qkDP6W4Si7FZgHiQ5B8HhiAU1");

#[account]
pub struct Contador {
    numero: u64,
    autoridad: Pubkey,
}

#[program]
mod programa_contador {
    use super::*;

    pub fn crear_contador(ctx: Context<Crear>, primer_numero: u64) -> Result<()> {
        ctx.accounts.contador.numero = primer_numero;
        ctx.accounts.contador.autoridad = ctx.accounts.autoridad.key();
        msg!("Creando un contador con número inicial {}", primer_numero);
        Ok(())
    }

    pub fn actualizar_contador(ctx: Context<Actualizar>, nuevo_numero: u64) -> Result<()> {
        ctx.accounts.contador.numero = nuevo_numero;
        msg!("Actualizando el contador a {}", nuevo_numero);
        Ok(())
    }

    pub fn eliminar_contador(_ctx: Context<Eliminar>) -> Result<()> {
        msg!("Eliminando el contador");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Crear<'info> {
    #[account(init, payer = autoridad, space = 8 + 8 + 32)]
    pub contador: Account<'info, Contador>,
    #[account(mut)]
    pub autoridad: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Actualizar<'info> {
    #[account(mut)]
    pub contador: Account<'info, Contador>,
}

#[derive(Accounts)]
pub struct Eliminar<'info> {
    #[account(mut, constraint = contador.autoridad == autoridad.key(), close = autoridad)]
    pub contador: Account<'info, Contador>,
    #[account(mut)]
    pub autoridad: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("No estas autorizado.")]
    NotAuthorized,
   
}
