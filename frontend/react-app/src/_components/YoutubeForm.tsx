import { useForm, FieldErrors } from 'react-hook-form';
import { yupResolver } from '@hookform/resolvers/yup';
import yup from '../_utils/yupJp';
import { TextField, } from '@mui/material';
import { getPlaylist } from '../_utils/getPlaylist';
import { url_to_playlistid, check_url } from '../_utils/regex';
import { Auth, Url, YoutubeList } from '../_utils/type';
import { Dispatch, SetStateAction, useEffect } from 'react';


const schema = yup.object({
    url: yup
        .string()
        .label('URL')
        .required()
        .url()
        .test("url",
            () => "再生リストのurlを指定してください",
            value => {
                const url: Url = { url: value };
                return check_url(url);
            }
        ),
})

type FormValues = Url;

export function YoutubeForm({ setSnippets, loading, setLoading, setIndex, auth }: { setSnippets: Dispatch<SetStateAction<YoutubeList | null>>, loading: boolean, setLoading: Dispatch<SetStateAction<boolean>>, setIndex: Dispatch<SetStateAction<number | null>>, auth: Auth }) {

    const { register, handleSubmit, getValues, formState: { errors } } = useForm<FormValues>({
        resolver: yupResolver(schema),
    });


    const onsubmit = async (data: FormValues) => {
        setLoading(true);
        setIndex(null);
        try {
            const res = await getPlaylist(url_to_playlistid(data));
            setSnippets(res);
        } catch (error) {
            console.error("API取得に失敗しました", error);
        } finally {
            setLoading(false);
        }
    };

    const onerror = (err: FieldErrors<FormValues>) => console.log(err);
    const onSubmit = handleSubmit(onsubmit, onerror)
    const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => { if (e.key === "Enter") onSubmit() };
    return (
        <TextField
            id="filled-error"
            label={errors.url?.message}
            {...register("url")}
            placeholder="https://www.youtube.com/playlist?list=LL"
            sx={{ width: "100%" }}
            error={Boolean(errors.url?.message)}
            onKeyDown={handleKeyDown}
            disabled={loading || auth === "logout"}
            size="small"
        />
    );
}