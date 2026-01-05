3

If you want to register a user without having them confirm the email address, you can just turn off the Confirm email setting from your Supabase dashboard under Authentication > Providers > Email > Confirm email.

Supabase dashboard

Share
Improve this answer
Follow
answered Jan 20, 2024 at 2:26
dshukertjr's user avatar
dshukertjr
19.1k1111 gold badges7070 silver badges101101 bronze badges
This option is not available for self hosted Supabase. How can we do this programatically? – 
Sheenergizer
 CommentedJun 5, 2024 at 14:40
1
@Sheenergizer You can set it by the auth.email.enable_confirmations value in the config.toml file. supabase.com/docs/guides/cli/… – 
dshukertjr
 CommentedJun 6, 2024 at 4:45
Add a comment
0

Here's how to do it. I needed this for e2e testing.

import { createClient } from '@supabase/supabase-js'

export  async function insertUser({ email, phone, password }) {
    const supabase = createClient(
        process.env.PUBLIC_SUPABASE_URL!,
        process.env.SUPABASE_SERVICE_KEY!,
    )
    await supabase.auth.signUp({
        email,
        password,
        options: { data: { phone } },
    })
    await supabase.auth.verifyOtp({
        type: 'signup',
        token: '123456',
        email,
    })
    return true
}
For the 123456 otp to be accepted, you need to add this trigger function to alter the confirmation token

create or replace function preset_otp()
    returns trigger
as $$
begin
    if (new.email = any(array['test@test.com'])) then
        new.confirmation_token := encode(sha224(concat(new.email,'123456')::bytea), 'hex');
        new.confirmation_sent_at := now() - interval '2 minutes';
    end if;
    return new;
end;
$$ language plpgsql;

create or replace trigger preset_otp before insert or update on auth.users for each row execute procedure public.preset_otp();